import os
import requests

# Mapping des notes vers leur numéro MIDI
note_to_midi = {
    'C': 0, 'Cs': 1, 'D': 2, 'Ds': 3, 'E': 4, 'F': 5, 'Fs': 6, 'G': 7, 'Gs': 8, 'A': 9, 'As': 10, 'B': 11
}

def note_to_midi_number(note_name):
    # Exemple: "A4" -> note="A", octave=4
    if len(note_name) == 3:  # Pour les notes avec dièse (ex: "Cs4")
        note, octave = note_name[:-1], int(note_name[-1])
    else:  # Pour les notes sans dièse (ex: "A4")
        note, octave = note_name[:-1], int(note_name[-1])
    
    # Calcul du numéro MIDI
    midi_number = (octave + 1) * 12 + note_to_midi[note]
    return midi_number

# Liste de tous les fichiers à télécharger
files = [
    'A2.flac', 'A3.flac', 'A4.flac', 'A5.flac',
    'As2.flac', 'As3.flac', 'As4.flac',
    'B2.flac', 'B3.flac', 'B4.flac',
    'C3.flac', 'C4.flac', 'C5.flac',
    'Cs3.flac', 'Cs4.flac', 'Cs5.flac',
    'D3.flac', 'D4.flac', 'D5.flac',
    'Ds3.flac', 'Ds4.flac', 'Ds5.flac',
    'E2.flac', 'E3.flac', 'E4.flac', 'E5.flac',
    'F2.flac', 'F3.flac', 'F4.flac', 'F5.flac',
    'Fs2.flac', 'Fs3.flac', 'Fs4.flac', 'Fs5.flac',
    'G2.flac', 'G3.flac', 'G4.flac', 'G5.flac',
    'Gs2.flac', 'Gs3.flac', 'Gs4.flac', 'Gs5.flac'
]

base_url = "https://raw.githubusercontent.com/Robhub/handpaner/74ee8563828745e42c7f8e596ce4330c9eec6a35/static/shellopan/"

# Télécharger et renommer chaque fichier
for file in files:
    url = base_url + file
    note_name = file[:-5]  # Enlever '.flac'
    midi_number = note_to_midi_number(note_name)
    new_name = f"{midi_number}.flac"
    
    print(f"Downloading {file} -> {new_name} (MIDI: {midi_number})")
    
    try:
        response = requests.get(url)
        response.raise_for_status()
        
        with open(new_name, 'wb') as f:
            f.write(response.content)
            
        print(f"Successfully downloaded and renamed to {new_name}")
    except Exception as e:
        print(f"Error processing {file}: {e}")

print("\nConversion terminée !")
