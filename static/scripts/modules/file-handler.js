export class FileHandler {
    constructor() {
        console.log('Initializing FileHandler...');
        this.dropZone = document.getElementById("drop-zone");
        this.fileInput = document.getElementById("file-input");
        this.fileNameDisplay = document.getElementById("file-name");
        this.eventListeners = [];
        
        if (!this.dropZone) {
            console.error('Drop zone element not found!');
        }
        if (!this.fileInput) {
            console.error('File input element not found!');
        }
        if (!this.fileNameDisplay) {
            console.error('File name display element not found!');
        }
        
        this.initializeEventListeners();
        console.log('FileHandler initialized');
    }



    initializeEventListeners() {
        if (this.dropZone) {
            // Bind event handlers once and store references
            this.boundHandleDragOver = this.handleDragOver.bind(this);
            this.boundHandleDragLeave = this.handleDragLeave.bind(this);
            this.boundHandleDrop = this.handleDrop.bind(this);
            this.boundHandleClick = () => this.fileInput?.click();

            // Add event listeners and store them for cleanup
            this.addListener(this.dropZone, "dragover", this.boundHandleDragOver);
            this.addListener(this.dropZone, "dragleave", this.boundHandleDragLeave);
            this.addListener(this.dropZone, "drop", this.boundHandleDrop);
            this.addListener(this.dropZone, "click", this.boundHandleClick);
        }

        if (this.fileInput) {
            this.boundHandleFileInputChange = this.handleFileInputChange.bind(this);
            this.addListener(this.fileInput, "change", this.boundHandleFileInputChange);
        }
    }

    updateFileName(name) {
        this.fileNameDisplay.textContent = name;
    }

    handleDragOver(e) {
        e.preventDefault();
        this.dropZone.classList.add("dragover");
    }

    handleDragLeave() {
        this.dropZone.classList.remove("dragover");
    }

    handleDrop(e) {
        console.log('File dropped');
        e.preventDefault();
        this.dropZone.classList.remove("dragover");

        const files = e.dataTransfer.files;
        if (files.length > 0) {
            console.log('Processing dropped file:', files[0].name);
            const file = files[0];
            if (this.validateFile(file)) {
                this.updateFileName(file.name);
                this.uploadFile(file);
            }
        }
    }

    handleFileInputChange() {
        if (this.fileInput.files.length > 0) {
            const file = this.fileInput.files[0];
            if (this.validateFile(file)) {
                this.updateFileName(file.name);
                this.uploadFile(file);
            }
        }
    }

    validateFile(file) {
        if (!file) {
            alert("Please select a file before uploading.");
            return false;
        }

        const fileExtension = file.name.split('.').pop().toLowerCase();
        if (fileExtension !== 'mscz') {
            alert("Invalid file type. Please upload a .mscz file.");
            return false;
        }

        return true;
    }

    uploadFile(file) {
        console.log('Starting file upload:', file.name);
        const formData = new FormData();
        formData.append('file', file);

        if (this.dropZone) {
            this.dropZone.classList.add('uploading');
        }

        fetch('/upload', {
            method: 'POST',
            body: formData
        })
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            return response.text();
        })
        .then(html => {
            console.log('Upload successful');
            // Injecter le contenu dans le container principal
            const container = document.querySelector('.main-content');
            if (!container) {
                console.error('Main content container not found');
                return;
            }

            // Créer un conteneur temporaire pour parser le HTML
            const tempContainer = document.createElement('div');
            tempContainer.innerHTML = html;

            // Extraire le contenu du upload-content
            const uploadContent = tempContainer.querySelector('.upload-content');
            if (!uploadContent) {
                console.error('Upload content not found in response');
                return;
            }

            // Injecter le contenu
            container.innerHTML = uploadContent.innerHTML;
            
            // Réinitialiser les scripts
            const scripts = container.getElementsByTagName('script');
            Array.from(scripts).forEach(script => {
                const newScript = document.createElement('script');
                Array.from(script.attributes).forEach(attr => {
                    newScript.setAttribute(attr.name, attr.value);
                });
                newScript.textContent = script.textContent;
                script.parentNode.replaceChild(newScript, script);
            });
        })
        .catch(error => {
            console.error('Upload error:', error);
            alert('Error uploading file. Please try again.');
        })
        .finally(() => {
            if (this.dropZone) {
                this.dropZone.classList.remove('uploading');
            }
        });
    }

    addListener(element, event, handler) {
        if (element && handler) {
            element.addEventListener(event, handler);
            this.eventListeners.push({ element, event, handler });
        }
    }

    cleanup() {
        console.log('Starting FileHandler cleanup...');
        // Remove all event listeners
        this.eventListeners.forEach(({ element, event, handler }) => {
            if (element) {
                element.removeEventListener(event, handler);
            }
        });
        this.eventListeners = [];

        // Clear file input
        if (this.fileInput) {
            this.fileInput.value = '';
        }

        // Clear filename display
        if (this.fileNameDisplay) {
            this.fileNameDisplay.textContent = '';
        }

        // Remove dragover class if present
        if (this.dropZone) {
            this.dropZone.classList.remove('dragover');
            this.dropZone.classList.remove('uploading');
        }

        console.log('FileHandler cleanup complete');
    }
}
