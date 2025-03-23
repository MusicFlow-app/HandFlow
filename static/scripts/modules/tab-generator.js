import { ScrollController } from './scroll-controller.js';
import { DisplayOptions } from './display-options.js';

export class TabGenerator {
    constructor() {
        this.scrollController = null;
        this.displayOptions = null;
        this.form = document.getElementById('generate-form');
        this.container = document.querySelector('#generate_container');
        this.stepConfiguration = document.getElementById('step-configuration');
        this.stepPlayback = document.getElementById('step-playback');
        
        if (this.form) {
            this.initializeForm();
        }
    }

    initializeForm() {
        this.form.addEventListener('submit', this.handleSubmit.bind(this));
        
        // Initialiser les listeners pour les changements de partie
        const partSelect = document.getElementById('part_id');
        if (partSelect) {
            partSelect.addEventListener('change', (e) => {
                const partName = e.target.options[e.target.selectedIndex].text;
                document.getElementById('part_name').value = partName;
            });
            // Définir la valeur initiale
            if (partSelect.options.length > 0) {
                document.getElementById('part_name').value = partSelect.options[0].text;
            }
        }

        // Initialiser le listener pour play_only_inscale
        const autoTranspose = document.getElementById('auto_transpose');
        if (autoTranspose) {
            autoTranspose.addEventListener('change', (e) => {
                document.getElementById('play_only_inscale').value = e.checked ? "1" : "0";
            });
        }
    }

    initializeControllers() {
        // Initialiser les contrôleurs seulement après la génération réussie
        this.scrollController = new ScrollController();
        this.displayOptions = new DisplayOptions();
    }

    async handleSubmit(e) {
        e.preventDefault();
        
        try {
            // Ajouter une classe pour indiquer le chargement
            this.form.classList.add('loading');
            
            const formData = new FormData(this.form);
            const response = await fetch('/generate', {
                method: 'POST',
                body: formData
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const html = await response.text();
            
            // Injecter le contenu généré
            if (this.container) {
                this.container.innerHTML = html;
                
                // Réinitialiser les scripts
                const scripts = this.container.getElementsByTagName('script');
                Array.from(scripts).forEach(script => {
                    const newScript = document.createElement('script');
                    Array.from(script.attributes).forEach(attr => {
                        newScript.setAttribute(attr.name, attr.value);
                    });
                    newScript.textContent = script.textContent;
                    script.parentNode.replaceChild(newScript, script);
                });

                // Initialiser les contrôleurs
                this.initializeControllers();

                // Passer à l'étape de lecture
                if (this.stepConfiguration && this.stepPlayback) {
                    this.stepConfiguration.classList.remove('active');
                    this.stepPlayback.classList.remove('hidden');
                    this.stepPlayback.classList.add('active');
                }
            }
        } catch (error) {
            console.error('Generation error:', error);
            alert('Error generating tablature. Please try again.');
        } finally {
            this.form.classList.remove('loading');
        }
    }

    cleanup() {
        if (this.scrollController) {
            this.scrollController.cleanup();
        }
        if (this.displayOptions) {
            this.displayOptions.cleanup();
        }
    }
}
