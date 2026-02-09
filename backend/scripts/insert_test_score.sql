-- Test Score for Celtic Handpan (9 notes)
-- Celtic scale: D3(50), A3(57), C4(60), D4(62), E4(64), F4(65), G4(67), A4(69), C5(72)
-- This creates a simple melody to test the Score Reader piano roll
-- Starts with scale runs (ascending + descending) for visual spacing testing

INSERT INTO tabs (id, file_size, metadata, score_data, favorite_count, created_at, last_used_at)
VALUES (
  'aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee'::uuid,
  1024,
  '{
    "work_title": "Celtic Test Melody",
    "composer": "HandFlow Test",
    "arranger": "Auto Generated",
    "key_signature": "Dmaj",
    "tempo": 90,
    "difficulty": 1,
    "category": 3
  }'::jsonb,
  '{
    "parts": [
      {
        "id": 1,
        "name": "Handpan",
        "measures": [
          {
            "id": 1,
            "time_signature": [4, 4],
            "chords": [
              [{"pitch": 50, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 57, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 60, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 62, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 64, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 65, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 67, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 69, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}]
            ]
          },
          {
            "id": 2,
            "time_signature": null,
            "chords": [
              [{"pitch": 72, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 69, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 67, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 65, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 64, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 62, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 60, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 57, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}]
            ]
          },
          {
            "id": 3,
            "time_signature": null,
            "chords": [
              [{"pitch": 50, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 57, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 60, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 62, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}]
            ]
          },
          {
            "id": 4,
            "time_signature": null,
            "chords": [
              [{"pitch": 64, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 65, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 67, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 69, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}]
            ]
          },
          {
            "id": 5,
            "time_signature": null,
            "chords": [
              [{"pitch": 72, "duration": "Half", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 69, "duration": "Half", "note_type": "Normal", "hand": "Left"}]
            ]
          },
          {
            "id": 6,
            "time_signature": null,
            "chords": [
              [{"pitch": 67, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 65, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 64, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 62, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}]
            ]
          },
          {
            "id": 7,
            "time_signature": null,
            "chords": [
              [{"pitch": 60, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 62, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 64, "duration": "Eighth", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 62, "duration": "Eighth", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 60, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 57, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}]
            ]
          },
          {
            "id": 8,
            "time_signature": null,
            "chords": [
              [{"pitch": 50, "duration": "Half", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 57, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 60, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}]
            ]
          },
          {
            "id": 9,
            "time_signature": null,
            "chords": [
              [{"pitch": 62, "duration": "Quarter", "note_type": "Ghost", "hand": "Left"}],
              [{"pitch": 64, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}],
              [{"pitch": 67, "duration": "Quarter", "note_type": "Normal", "hand": "Left"}],
              [{"pitch": 69, "duration": "Quarter", "note_type": "Normal", "hand": "Right"}]
            ]
          },
          {
            "id": 10,
            "time_signature": null,
            "chords": [
              [{"pitch": 50, "duration": "Whole", "note_type": "Normal", "hand": "Right"}]
            ]
          }
        ]
      }
    ]
  }'::jsonb,
  0,
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
)
ON CONFLICT (id) DO UPDATE SET
  metadata = EXCLUDED.metadata,
  score_data = EXCLUDED.score_data,
  last_used_at = CURRENT_TIMESTAMP;

-- Verify insertion
SELECT id, metadata->>'work_title' as title, metadata->>'tempo' as tempo,
       jsonb_array_length(score_data->'parts'->0->'measures') as measure_count
FROM tabs WHERE id = 'aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee';
