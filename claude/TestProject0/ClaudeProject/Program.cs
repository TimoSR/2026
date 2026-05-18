using System;
using System.Collections.Generic;
using System.Linq;

// --- Entry Point (top-level statements must come first) ---

var repo = new InMemoryRepository<AppTask>();
var service = new TaskService(repo);

// Seed data
var seeds = new (string name, string desc, Priority priority)[]
{
    ("Fix login bug",      "Users cannot log in with SSO",          Priority.Critical),
    ("Add dark mode",      "Implement theme toggle in settings",    Priority.Medium),
    ("Write unit tests",   "Cover the auth module",                 Priority.High),
    ("Update README",      "Document new API endpoints",            Priority.Low),
    ("Optimize DB query",  "Slow query on the reports page",        Priority.High),
    ("Deploy to staging",  "Release v2.3 to staging environment",   Priority.Medium),
};

foreach (var (name, desc, priority) in seeds)
    service.Create(name, desc, priority);

service.Complete(1);
service.Complete(4);

Console.WriteLine("=== Pending Tasks (by priority) ===");
foreach (var task in service.GetPending())
    Console.WriteLine($"  {task}");

Console.WriteLine("\n=== All Tasks Grouped by Priority ===");
foreach (var group in service.GroupByPriority())
{
    Console.WriteLine($"\n  [{group.Key}]");
    foreach (var task in group)
        Console.WriteLine($"    {task}");
}

Console.WriteLine("\n=== Stats ===");
foreach (var (priority, count) in service.GetStats().OrderByDescending(kv => kv.Key))
    Console.WriteLine($"  {priority}: {count} task(s)");

int total     = repo.GetAll().Count();
int completed = repo.GetAll().Count(t => t.IsCompleted);
Console.WriteLine($"\n  Total: {total}  |  Completed: {completed}  |  Pending: {total - completed}");

Console.WriteLine("\n=== Sending Alerts for Critical Tasks ===");
var criticalPending = service.GetPending().Where(t => t.Priority == Priority.Critical).ToList();

if (criticalPending.Count == 0)
{
    Console.WriteLine("  No critical tasks pending.");
}
else
{
    var notifications = criticalPending
        .Select(t => Notifier.SendAsync("team@example.com", $"CRITICAL task pending: {t.Name}"));
    await Task.WhenAll(notifications);
}

Console.WriteLine("\nDone.");

// --- Domain ---

enum Priority { Low, Medium, High, Critical }

class AppTask : IEntity
{
    public int Id { get; init; }
    public string Name { get; init; }
    public string Description { get; set; }
    public Priority Priority { get; set; }
    public bool IsCompleted { get; private set; }
    public DateTime CreatedAt { get; } = DateTime.UtcNow;
    public DateTime? CompletedAt { get; private set; }

    public AppTask(int id, string name, string description, Priority priority)
    {
        Id = id;
        Name = name;
        Description = description;
        Priority = priority;
    }

    public void Complete()
    {
        IsCompleted = true;
        CompletedAt = DateTime.UtcNow;
    }

    public override string ToString() =>
        $"[{Priority}] #{Id} {Name} — {(IsCompleted ? $"Done at {CompletedAt:t}" : "Pending")}";
}

// --- Interfaces ---

interface IEntity
{
    int Id { get; }
    string Name { get; }
}

interface IRepository<T> where T : IEntity
{
    void Add(T item);
    T? GetById(int id);
    IEnumerable<T> GetAll();
    bool Remove(int id);
}

// --- Generic In-Memory Repository ---

class InMemoryRepository<T> : IRepository<T> where T : IEntity
{
    private readonly Dictionary<int, T> _store = new();

    public void Add(T item)
    {
        if (_store.ContainsKey(item.Id))
            throw new InvalidOperationException($"Item with Id {item.Id} already exists.");
        _store[item.Id] = item;
    }

    public T? GetById(int id) => _store.TryGetValue(id, out var item) ? item : default;
    public IEnumerable<T> GetAll() => _store.Values;
    public bool Remove(int id) => _store.Remove(id);
}

// --- Service Layer ---

class TaskService(IRepository<AppTask> repo)
{
    private int _nextId = 1;

    public AppTask Create(string name, string description, Priority priority)
    {
        var task = new AppTask(_nextId++, name, description, priority);
        repo.Add(task);
        return task;
    }

    public bool Complete(int id)
    {
        var task = repo.GetById(id);
        if (task is null) return false;
        task.Complete();
        return true;
    }

    public IEnumerable<AppTask> GetPending() =>
        repo.GetAll().Where(t => !t.IsCompleted).OrderByDescending(t => t.Priority);

    public IEnumerable<IGrouping<Priority, AppTask>> GroupByPriority() =>
        repo.GetAll().GroupBy(t => t.Priority).OrderByDescending(g => g.Key);

    public Dictionary<Priority, int> GetStats() =>
        repo.GetAll()
            .GroupBy(t => t.Priority)
            .ToDictionary(g => g.Key, g => g.Count());
}

// --- Async Notifier ---

static class Notifier
{
    public static async Task SendAsync(string recipient, string message)
    {
        await Task.Delay(50);
        Console.WriteLine($"  [Notifier] -> {recipient}: \"{message}\"");
    }
}
