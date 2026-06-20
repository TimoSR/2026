#include <cassert>
#include <iostream>
#include <memory>
#include <string>

using namespace std;

#include "smart_pointers.cpp"

int main() {
    unique_ptr<Song> draft = createDraftSong("Draft Song");
    string draftTitle = draft->getTitle();
    assert(draftTitle == "Draft Song");

    shared_ptr<Song> shared = make_shared<Song>("Shared Song");
    Playlist one{};
    Playlist two{};

    one.addSong(shared);
    two.addSong(shared);

    int firstPlaylistCount = one.songCount();
    int secondPlaylistCount = two.songCount();
    string firstPlaylistFirstSong = one.firstSongTitleOrEmpty();
    assert(firstPlaylistCount == 1);
    assert(secondPlaylistCount == 1);
    assert(firstPlaylistFirstSong == "Shared Song");

    cout << "smart_pointers tests passed\n";
    return 0;
}
