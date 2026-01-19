using System;
using System.Globalization;

namespace HackerRank.ProblemSolving.Time;

public class TimeConversion
{
    // New solution
    //D2 adds a 0 in front of the number, if it does not take all positions
    public static string timeConversion(string s)
    {
        s = s.Trim();
        var prefix = s.Substring(s.Length - 2);
        var time = s.Remove(s.Length - 2);
        var values = time.Split(':');
        var hours = int.Parse(values[0]);
        var minutes = int.Parse(values[1]);
        var seconds = int.Parse(values[2]);
        
        if (hours == 12 && prefix == "AM") hours = 0; 
        if (hours != 12 && prefix == "PM") hours += 12;

        var converted_time = $"{hours:D2}:{minutes:D2}:{seconds:D2}";
        
        return converted_time;
    }
    
    // I have forgotten how to work with strings it seems
    public static string Old_timeConversion(string s)
    {
        s = s.Trim();
        var prefix = s.Substring(s.Length - 2);
        var time = s.Remove(s.Length - 2);
        var values = time.Split(":");
        var hours = int.Parse(values[0]);
        var minutes = int.Parse(values[1]);
        var seconds = int.Parse(values[2]);

        if (prefix == "AM")
        {
            if (hours == 12)
            {
                hours = 00;
            }
        }

        if (prefix == "PM")
        {
            if (hours != 12)
            {
                hours += 12;
            }
        }

        return $"{hours:D2}:{minutes:D2}:{seconds:D2}";
    }
    
    //OLD GPT
    
    public static string OLD_GPT_timeConversion(string s)
    {
        // Get the last two characters (AM/PM) using the ^ operator
        var prefix = s[^2..]; // equivalent to s.Substring(s.Length - 2)
    
        // Get the string excluding the last two characters (time part) using the .. operator
        var time = s[..^2]; // equivalent to s.Remove(s.Length - 2)
    
        var values = time.Split(':');

        int hour = int.Parse(values[0]);
        int minute = int.Parse(values[1]);
        int second = int.Parse(values[2]);

        if (prefix == "AM" && hour == 12)
        {
            hour = 00;
        }
        else if (prefix == "PM" && hour != 12)
        {
            hour += 12;
        }

        return $"{hour:D2}:{minute:D2}:{second:D2}";
    }
    
    //GPT
    public static string GPT_TimeConversion(string s)
    {
        if (s is null) throw new ArgumentNullException(nameof(s));

        // Input example: "07:05:45PM"
        // Correct format string: hh:mm:sstt
        var dt = DateTime.ParseExact(
            s.Trim(),
            "hh:mm:sstt",
            CultureInfo.InvariantCulture,
            DateTimeStyles.None
        );

        return dt.ToString("HH:mm:ss", CultureInfo.InvariantCulture);
    }
}