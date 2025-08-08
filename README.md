# rember

I forget things all the time. This is a tool to help me record short notes of ideas, thoughts, events, and other things that I can reference later on.

## Backend

### Communication interface

The server communicates with clients through websockets. Message format is as follows:

```json
{
    "type": "message_type",
    "data": {
        // actual data, whose format is determined by the message type
    }
}
```

The specific data format depends on the message type. See `backend/src/engine/core.rs` for the list of supported message types.

### Data model

The core data is stored as a collection of notes. Each note has:
- some text content
- zero or more tags
- an inception date
- zero or more additional labeled dates
- zero or more linked notes

### Search

Search is the primary usecase for the app. Notes can be searched by:
- tags (fuzzy)
- text content (fuzzy)
- dates (range?)
- links from a given note
