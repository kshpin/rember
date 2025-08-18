-- PARAMETERS
-- $1: search_text          -> The text for fuzzy searching (e.g., 'databas desin').
-- $2: search_tags          -> An array of tag names to match (e.g., ARRAY['project', 'idea']).

WITH notes_with_tags AS (
    SELECT
        nt.note_id,
        -- Aggregate all tag names for a note into a single array (postgres specific)
        array_agg(t.name) as tags
    FROM note_tags nt
    JOIN tags t ON nt.tag_id = t.id
    GROUP BY nt.note_id
)
SELECT
    n.id,
    n.text,
    n.created_at
FROM
    notes AS n
JOIN
    notes_with_tags nwt ON n.id = nwt.note_id
WHERE
    -- 1. Check if the note's tag array contains all the search tags
    nwt.tags @> $2
    -- 2. Perform the fuzzy text search
    AND n.text % $1
ORDER BY
    -- 3. Sort by similarity score
    similarity(n.text, $1) DESC;
