use diesel::prelude::*;
use crate::schema::*;

use super::Track;

#[derive(Queryable)]
/// A playlist in the database
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = playlists)]
/// A playlist that is not yet in the database
pub struct NewPlaylist {
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>
}

#[derive(Queryable)]
/// A playlist track in the database
pub struct PlaylistTrack {
    pub playlist_id: i32,
    pub track_id: i32,
    pub track_number: i32
}

#[derive(Insertable)]
#[diesel(table_name = playlist_tracks)]
/// A playlist track that is not yet in the database
pub struct NewPlaylistTrack {
    pub playlist_id: i32,
    pub track_id: i32,
    pub track_number: i32
}

impl Playlist {
    /// Get all playlists
    pub fn all(conn: &mut SqliteConnection) -> QueryResult<Vec<Playlist>> {
        playlists::table.load::<Playlist>(conn)
    }

    /// Get a playlist by id
    pub fn get(id: i32, conn: &mut SqliteConnection) -> QueryResult<Playlist> {
        playlists::table.find(id).get_result::<Playlist>(conn)
    }

    /// Insert a new playlist
    pub fn insert(playlist: NewPlaylist, conn: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(playlists::table)
            .values(&playlist)
            .execute(conn)
    }

    /// Delete a playlist
    pub fn delete(id: i32, conn: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::delete(playlists::table.find(id))
            .execute(conn)
    }

    /// Get all tracks in a playlist
    pub fn get_tracks(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<Track>> {
        playlist_tracks::table.filter(playlist_tracks::playlist_id.eq(self.id))
            .inner_join(
                tracks::table.on(
                    playlist_tracks::track_id.eq(tracks::local_id)
                )
            )
            .select(tracks::all_columns)
            .order(playlist_tracks::track_number)
            .load::<Track>(conn)
    }
}

impl PlaylistTrack {
    /// Insert a new playlist track
    pub fn insert(playlist_track: NewPlaylistTrack, conn: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(playlist_tracks::table)
            .values(&playlist_track)
            .execute(conn)
    }

    /// Delete a playlist track
    pub fn delete(playlist_id: i32, track_id: i32, conn: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::delete(playlist_tracks::table.filter(
            playlist_tracks::playlist_id.eq(playlist_id)
                .and(playlist_tracks::track_id.eq(track_id))
        ))
        .execute(conn)
    }

    /// Get the corresponding track
    pub fn get_track(&self, conn: &mut SqliteConnection) -> QueryResult<Track> {
        tracks::table.find(self.track_id)
            .get_result::<Track>(conn)
    }

    /// Get the corresponding playlist
    pub fn get_playlist(&self, conn: &mut SqliteConnection) -> QueryResult<Playlist> {
        playlists::table.find(self.playlist_id)
            .get_result::<Playlist>(conn)
    }
}
