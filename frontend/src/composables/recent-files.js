// Module for handling recent files functionality
export class RecentFilesHandler {
    constructor() {
        this.initializeEventListeners();
    }

    initializeEventListeners() {
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('reuse-file')) {
                this.handleReuseFile(e.target);
            }
        });
    }

    async handleReuseFile(button) {
        const fileId = button.dataset.fileId;
        if (!fileId) return;

        try {
            const response = await fetch(`/upload?file_id=${fileId}`, {
                method: 'GET',
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const content = await response.text();
            
            // Following HandFlow's template injection pattern
            const tempContainer = document.createElement('div');
            tempContainer.innerHTML = content;
            
            const uploadContent = tempContainer.querySelector('.upload-content');
            if (!uploadContent) {
                console.error('Could not find .upload-content in the response');
                return;
            }

            const mainContent = document.querySelector('.main-content');
            if (!mainContent) {
                console.error('Could not find .main-content container');
                return;
            }

            mainContent.innerHTML = uploadContent.outerHTML;
            
            // Re-initialize any scripts specific to upload_tmpl.html
            // This follows HandFlow's script reinitialization pattern
            const event = new CustomEvent('uploadContentLoaded');
            document.dispatchEvent(event);

        } catch (error) {
            console.error('Error reusing file:', error);
        }
    }
}

// Initialize when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new RecentFilesHandler();
});
