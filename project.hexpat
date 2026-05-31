#pragma endian little
#pragma description "Kreniq project file (.knq)"

// ─── Primitives ──────────────────────────────────────────────────────────────

struct Color32 {
    u8 r, g, b, a;
};

// ─── Note (24 bytes fixed) ───────────────────────────────────────────────────

struct Note {
    double start;
    double duration;
    float  pitch;
    float  velocity;
};

// ─── NoteRegion ──────────────────────────────────────────────────────────────
// notes blob: [note_id: u64][Note: 24 bytes] × N  (32 bytes per entry)

struct NoteEntry {
    u64  note_id;
    Note note;
};

struct NoteRegion {
    double   start;
    double   duration;
    u64      notes_len;
    NoteEntry notes[notes_len / 32];
};

// ─── NoteTrack ───────────────────────────────────────────────────────────────
// regions blob order: region_data_len, region_id, NoteRegion data

struct NoteRegionEntry {
    u64        region_data_len;
    u64        region_id;
    NoteRegion region;
};

struct NoteTrack {
    u64              regions_len;
    NoteRegionEntry  regions[while($ < addressof(regions_len) + 8 + regions_len)];
};

// ─── Node ────────────────────────────────────────────────────────────────────
// kind: 0=NoteInput  1=AudioInput  2=AudioOutput  3=KaslNode
// For KaslNode the node bytes are: [u64 kasl_path_len][kasl_path bytes]

struct Node {
    u8 kind;
    if (kind == 3) {
        u64  kasl_path_len;
        char kasl_path[kasl_path_len];
    }
};

struct NodeEntry {
    u64  node_id;
    u64  node_data_len;
    Node node;
};

// ─── Graph ───────────────────────────────────────────────────────────────────

struct Edge {
    u64 from_id;
    u64 out_idx;
    u64 to_id;
    u64 in_idx;
};

struct Graph {
    u64       input_id;
    u64       output_id;
    u64       nodes_len;
    NodeEntry nodes[while($ < addressof(nodes_len) + 8 + nodes_len)];
    u64       edge_count;
    Edge      edges[edge_count];
};

// ─── Track ───────────────────────────────────────────────────────────────────
// track_kind: 0=AudioTrack  1=NoteTrack
// Track layout: [u64 graph_len][Graph data][u8 track_kind][u64 track_content_len][track_content bytes]
// track_content bytes contain the serialized content for audio or note tracks.
// (NoteTrack content itself begins with a regions_len u64 and then region entries.)

struct Track {
    u64 graph_len;
    Graph graph;
    u8  track_kind;
    u64 track_content_len;
    u8  track_content[track_content_len];
};

struct TrackEntry {
    u64   track_id;
    u64   track_data_len;
    Track track;
};

// ─── TempoMap ────────────────────────────────────────────────────────────────
// TempoEvent is 24 bytes fixed: beat(f64) + bpm(f64) + sample_offset(u64)

struct TempoEvent {
    double beat;
    double bpm;
    u64    sample_offset;
};

struct TempoMap {
    u64        events_len;
    TempoEvent events[events_len / 24];
};

// ─── AudioContext (32 bytes) ─────────────────────────────────────────────────

struct AudioContext {
    u64 channels;
    u64 sample_rate;
    u64 buffer_size;
    u64 max_voices;
};

// ─── Project ─────────────────────────────────────────────────────────────────

struct Project {
    AudioContext audio_ctx;
    double       range_start;
    double       range_duration;
    u64          tempo_map_len;
    TempoMap     tempo_map;
    u64          tracks_len;
    TrackEntry   tracks[while($ < addressof(tracks_len) + 8 + tracks_len)];
};

// ─── Project Metadata (StoredProjMeta) ───────────────────────────────────────

// StoredNodeMeta: serialized as:
// [u8 node_type][u64 display_name_len][display_name bytes][f32 pos_x][f32 pos_y]
struct StoredNodeMeta {
    u8  node_type;
    u64 name_len;
    char name[name_len];
    float pos_x;
    float pos_y;
};

struct StoredNodeEntry {
    u64             node_id;
    u64             node_data_len;
    StoredNodeMeta  node_meta;
};

// StoredGraphMeta: serialized as:
// [u64 nodes_bytes_len][ then repeated: (node_id:u64)(node_bytes_len:u64)(StoredNodeMeta bytes) ]
struct StoredGraphMeta {
    u64              nodes_len;
    StoredNodeEntry  nodes[while($ < addressof(nodes_len) + 8 + nodes_len)];
};

// Region meta entry in track metadata
struct RegionMetaEntry {
    u64  region_id;
    u64  region_meta_len;
    char region_name[region_meta_len];
};

// StoredTrackMeta now includes the node graph layout (length-prefixed)
struct StoredTrackMeta {
    Color32         color;
    u64             name_len;
    char            name[name_len];
    u64             region_metas_len;
    RegionMetaEntry region_metas[while($ < addressof(region_metas_len) + 8 + region_metas_len)];
    u64             node_graph_len;
    StoredGraphMeta node_graph;
};

struct TrackMetaEntry {
    u64            track_id;
    u64            track_meta_len;
    StoredTrackMeta track_meta;
};

struct KaslPathEntry {
    u64  path_len;
    char path[path_len];
};

struct StoredProjMeta {
    u64             track_metas_len;
    TrackMetaEntry  track_metas[while($ < addressof(track_metas_len) + 8 + track_metas_len)];
    u64             kasl_search_paths_len;
    KaslPathEntry   kasl_search_paths[while($ < addressof(kasl_search_paths_len) + 8 + kasl_search_paths_len)];
};

// ─── File Root ───────────────────────────────────────────────────────────────

struct KreniqFile {
    char         magic[6];          // "KNODIQ"
    u32          version_major;
    u32          version_minor;
    u32          version_patch;
    u64          proj_meta_len;
    StoredProjMeta proj_meta;
    Project      project;
};

KreniqFile file @ 0x00;
