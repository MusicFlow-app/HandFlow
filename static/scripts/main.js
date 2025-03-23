import { AudioController } from './modules/audio-controller.js';
import { DisplayController } from './modules/display-controller.js';
import { FileHandler } from './modules/file-handler.js';
import { StepsController } from './modules/steps-controller.js';
import { RecentFilesHandler } from './modules/recent-files.js';
import { TabGenerator } from './modules/tab-generator.js';
import { LibraryFilter } from './modules/library-filter.js';

export class App {
    constructor() {
        console.log('Initializing App...');
        // Initialize controllers first
        this.initializeControllers();
        // Then set up event listeners
        this.initializeEventListeners();
        // Finally, load initial state
        this.loadInitialState();
        console.log('App initialized');
    }

    initializeControllers() {
        try {
            this.libraryFilter = new LibraryFilter();
            console.log('Initializing controllers...');
            this.audioController = new AudioController();
            this.displayController = new DisplayController();
            this.fileHandler = new FileHandler();
            this.stepsController = new StepsController();
            this.recentFilesHandler = new RecentFilesHandler();
            this.tabGenerator = new TabGenerator();
            console.log('Controllers initialized');
        } catch (error) {
            console.error('Error initializing controllers:', error);
        }
    }

    initializeEventListeners() {
        try {
            console.log('Setting up event listeners...');
            // Handle cleanup on page unload
            const cleanupHandler = () => this.cleanup();
            window.addEventListener('beforeunload', cleanupHandler);
            this.cleanupHandler = cleanupHandler; // Store reference for cleanup

            // Listen for piece loaded events
            const pieceLoadedHandler = (e) => {
                console.log('Piece loaded:', e.detail);
            };
            document.addEventListener('pieceLoaded', pieceLoadedHandler);
            this.pieceLoadedHandler = pieceLoadedHandler; // Store reference for cleanup

            console.log('Event listeners set up');
        } catch (error) {
            console.error('Error setting up event listeners:', error);
        }
    }

    async loadInitialState() {
        try {
            console.log('Loading initial state...');
            const pieceInfo = document.querySelector('.piece-info');
            if (pieceInfo) {
                document.dispatchEvent(new CustomEvent('pieceLoaded', {
                    detail: {
                        title: pieceInfo.querySelector('h3')?.textContent || '',
                        composer: pieceInfo.querySelector('.composer')?.textContent || '',
                        arranger: pieceInfo.querySelector('.arranger')?.textContent || ''
                    }
                }));
            }
            console.log('Initial state loaded');
        } catch (error) {
            console.error('Error loading initial state:', error);
        }
    }

    cleanup() {
        try {
            console.log('Starting App cleanup...');
            // Clean up controllers
            if (this.audioController && typeof this.audioController.cleanup === 'function') {
                this.audioController.cleanup();
            }
            if (this.displayController && typeof this.displayController.cleanup === 'function') {
                this.displayController.cleanup();
            }
            if (this.fileHandler && typeof this.fileHandler.cleanup === 'function') {
                this.fileHandler.cleanup();
            }
            if (this.stepsController && typeof this.stepsController.cleanup === 'function') {
                this.stepsController.cleanup();
            }
            if (this.recentFilesHandler && typeof this.recentFilesHandler.cleanup === 'function') {
                this.recentFilesHandler.cleanup();
            }
            if (this.tabGenerator && typeof this.tabGenerator.cleanup === 'function') {
                this.tabGenerator.cleanup();
            }

            // Clean up event listeners
            if (this.cleanupHandler) {
                window.removeEventListener('beforeunload', this.cleanupHandler);
            }
            if (this.pieceLoadedHandler) {
                document.removeEventListener('pieceLoaded', this.pieceLoadedHandler);
            }

            console.log('App cleanup complete');
        } catch (error) {
            console.error('Error during cleanup:', error);
        }
    }
}