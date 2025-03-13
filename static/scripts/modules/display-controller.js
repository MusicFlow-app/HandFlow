export class DisplayController {
    constructor() {
        console.log('Initializing DisplayController...');
        this.inlineDisplay = true;
        this.showRestColor = true;
        this.showSvg = true;
        this.eventListeners = [];
        this.initializeEventListeners();
        console.log('DisplayController initialized');
    }

    initializeEventListeners() {
        // Helper to safely add event listeners
        const addListener = (id, event, handler) => {
            const element = document.getElementById(id);
            if (element) {
                const boundHandler = handler.bind(this);
                element.addEventListener(event, boundHandler);
                this.eventListeners.push({ element, event, handler: boundHandler });
            } else {
                console.warn(`Element with id '${id}' not found`);
            }
        };

        // Size controls
        addListener('decrease', 'click', () => this.changeHandpanSize('decrease'));
        addListener('increase', 'click', () => this.changeHandpanSize('increase'));

        // Display toggles
        addListener('inlineDisplay', 'change', (e) => {
            this.inlineDisplay = e.target.checked;
            this.updateDisplaySettings();
        });

        addListener('showRestColor', 'change', (e) => {
            this.showRestColor = e.target.checked;
            this.updateDisplaySettings();
        });

        addListener('showSvg', 'change', (e) => {
            this.showSvg = e.target.checked;
            this.updateDisplaySettings();
        });
    }

    changeHandpanSize(action) {
        document.dispatchEvent(new CustomEvent('changeHandpanSize', {
            detail: { action }
        }));
    }

    updateDisplaySettings() {
        document.dispatchEvent(new CustomEvent('displaySettingsChange', {
            detail: {
                inlineDisplay: this.inlineDisplay,
                showRestColor: this.showRestColor,
                showSvg: this.showSvg
            }
        }));
    }

    cleanup() {
        console.log('Cleaning up DisplayController...');
        // Remove all event listeners
        this.eventListeners.forEach(({ element, event, handler }) => {
            element.removeEventListener(event, handler);
        });
        this.eventListeners = [];
        console.log('DisplayController cleanup complete');
    }
}
