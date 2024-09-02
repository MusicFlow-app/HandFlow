document.addEventListener("DOMContentLoaded", function () {
    const dropZone = document.getElementById("drop-zone");
    const fileInput = document.getElementById("file-input");
    const fileNameDisplay = document.getElementById("file-name");

    // Utility function to update file name display
    const updateFileName = (name) => {
        fileNameDisplay.textContent = name;
    };

    // Handle drag over event
    const handleDragOver = (e) => {
        e.preventDefault();
        dropZone.classList.add("dragover");
    };

    // Handle drag leave event
    const handleDragLeave = () => {
        dropZone.classList.remove("dragover");
    };

    // Handle drop event
    const handleDrop = (e) => {
        e.preventDefault();
        dropZone.classList.remove("dragover");

        const files = e.dataTransfer.files;
        if (files.length > 0) {
            fileInput.files = files;
            updateFileName(files[0].name);
        }
    };

    // Handle file input change event
    const handleFileInputChange = () => {
        if (fileInput.files.length > 0) {
            updateFileName(fileInput.files[0].name);
        }
    };

    // Attach event listeners
    dropZone.addEventListener("dragover", handleDragOver);
    dropZone.addEventListener("dragleave", handleDragLeave);
    dropZone.addEventListener("drop", handleDrop);
    dropZone.addEventListener("click", () => fileInput.click());
    fileInput.addEventListener("change", handleFileInputChange);
});

// Validate the file type before form submission
function validateFile() {
    const fileInput = document.getElementById("file-input");

    if (fileInput.files.length === 0) {
        alert("Please select a file before uploading.");
        return false;
    }

    const fileName = fileInput.files[0].name;
    const fileExtension = fileName.split('.').pop().toLowerCase();

    if (fileExtension !== 'mscz') {
        alert("Invalid file type. Please upload a .mscz file.");
        return false;
    }

    return true;
}