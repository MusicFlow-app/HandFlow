# HandFlow Backend API Documentation

The HandFlow backend is built with Rust and provides a robust API for handling music tablature processing and generation.

## API Routes

### File Upload
- **POST** `/upload`
  - Accepts MusicXML files
  - Returns processed template with piece information
  - Handles file validation and parsing

### Tablature Generation
- **POST** `/generate`
  - Accepts configuration parameters:
    - `scale`: Selected handpan scale
    - `transpose`: Transposition value
    - `part`: Selected musical part
  - Returns generated tablature in HTML format

### Recent Files
- **GET** `/recent`
  - Returns list of recently processed files
  - Includes metadata (title, composer, date)

### Favorites
- **POST** `/favorites/toggle/:id`
  - Toggle favorite status for a specific piece
- **GET** `/favorites`
  - Retrieve list of favorited pieces

## Technical Capabilities

### MusicXML Processing
- Parse and validate MusicXML files
- Extract musical information (notes, rhythm, parts)
- Handle multiple instrument parts

### Handpan Adaptation
- Scale mapping and note transposition
- Rhythm pattern recognition
- Optimal note placement for handpan

### Template Engine
- Dynamic HTML template rendering
- Responsive tablature generation
- Support for multiple view formats

### File Management
- Temporary file storage
- Recent files tracking
- Favorites system
- File cleanup routines

### Error Handling
- Comprehensive error responses
- Input validation
- File format verification
- Graceful failure handling
