-- TABLE: Notes
-- Stores all text entries created by the user. Each note is timestamped.
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),           -- Unique identifier for the note
    text TEXT NOT NULL,                                      -- Main body of the note
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP  -- When the note was created
);

-- Create trigram index for fuzzy/typo-tolerant search on notes.text
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX notes_text_trgm_idx ON notes USING GIN (text gin_trgm_ops);


-- TABLE: Tags
-- Represents reusable tags for categorizing notes (e.g. #idea, #journal)
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),           -- Unique identifier for the tag
    name TEXT UNIQUE NOT NULL                                -- Tag name (e.g. 'idea', 'project', 'dream')
);

-- Create trigram index for fuzzy/typo-tolerant search on tags.name
-- disabled for now because it's not real fuzzy search
-- CREATE INDEX tags_name_trgm_idx ON tags USING GIN (name gin_trgm_ops);


-- TABLE: Note_Tags
-- Many-to-many relationship between notes and tags
CREATE TABLE note_tags (
    note_id UUID NOT NULL REFERENCES notes(id) ON DELETE CASCADE,  -- Linked note
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,    -- Linked tag
    PRIMARY KEY (note_id, tag_id)                                  -- Ensures each pair is unique
);


-- TABLE: Note_Dates
-- Optional date associations for a note (e.g., event date, future reference, etc.)
CREATE TABLE note_dates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),                   -- Unique date reference
    note_id UUID NOT NULL REFERENCES notes(id) ON DELETE CASCADE,    -- Associated note
    label TEXT,                                                      -- Optional label (e.g., 'event', 'deadline', 'reminder')
    date DATE NOT NULL                                               -- The actual date being referenced
);


-- TABLE: Note_Links
-- Represents bidirectional links between notes.
-- Enforces that each link exists only once, regardless of order (e.g., (1,5) == (5,1))
CREATE TABLE note_links (
    note1_id UUID NOT NULL REFERENCES notes(id) ON DELETE CASCADE,  -- One note in the pair
    note2_id UUID NOT NULL REFERENCES notes(id) ON DELETE CASCADE,  -- The other note in the pair
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,                 -- When the link was created
    PRIMARY KEY (note1_id, note2_id),                               -- Enforce uniqueness of each link
    CHECK (note1_id < note2_id)                                     -- Prevents (A,B) and (B,A) duplicates
);
