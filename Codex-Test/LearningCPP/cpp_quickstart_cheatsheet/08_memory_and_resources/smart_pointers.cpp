#include <memory>
#include <string>
#include <vector>

using namespace std;

class Song {
private:
    string title;

public:
    Song(string titleValue) : title(titleValue) {
    }

    string getTitle() const {
        return title;
    }
};

unique_ptr<Song> createDraftSong(string title) {
    // unique_ptr = one owner.
    return make_unique<Song>(title);
}

class Playlist {
private:
    vector<shared_ptr<Song>> songs;

public:
    void addSong(shared_ptr<Song> song) {
        songs.push_back(song);
    }

    int songCount() const {
        return static_cast<int>(songs.size());
    }

    string firstSongTitleOrEmpty() const {
        if (songs.empty()) {
            return "";
        }
        return songs[0]->getTitle();
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    unique_ptr<Song> draftSong = createDraftSong("First Idea");
    cout << draftSong->getTitle() << "\n";

    shared_ptr<Song> sharedSong = make_shared<Song>("Team Favorite");
    Playlist morningPlaylist{};
    Playlist eveningPlaylist{};

    morningPlaylist.addSong(sharedSong);
    eveningPlaylist.addSong(sharedSong);

    cout << morningPlaylist.songCount() << "\n";
    cout << eveningPlaylist.songCount() << "\n";
    return 0;
}
#endif
