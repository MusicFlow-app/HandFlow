/**
 * Contrôleur pour la gestion des étapes de l'interface HandFlow
 */
export class StepsController {
    constructor() {
        this.generateForm = document.getElementById('generate-form');
        this.playbackStep = document.getElementById('step-playback');
        this.bindEvents();
    }

    /**
     * Initialise les écouteurs d'événements
     */
    bindEvents() {
        if (this.generateForm) {
            this.generateForm.addEventListener('submit', this.handleGeneration.bind(this));
        }
    }

    /**
     * Gère la soumission du formulaire de génération
     * @param {Event} event - L'événement de soumission
     */
    handleGeneration(event) {
        event.preventDefault();
        const formData = new FormData(this.generateForm);

        // Désactive le formulaire pendant la génération
        const submitButton = this.generateForm.querySelector('button[type="submit"]');
        if (submitButton) {
            submitButton.disabled = true;
            submitButton.innerHTML = `
                <svg class="loading-icon" viewBox="0 0 24 24">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z"/>
                </svg>
                <span>Génération en cours...</span>
            `;
        }

        fetch('/generate', {
            method: 'POST',
            body: formData
        })
        .then(response => response.text())
        .then(html => {
            // Injecte le contenu généré
            const generateContainer = document.getElementById('generate-container');
            if (generateContainer) {
                generateContainer.innerHTML = html;
                
                // Affiche l'étape 2 avec une animation
                if (this.playbackStep) {
                    this.playbackStep.classList.remove('hidden');
                    // Force un reflow pour activer la transition
                    void this.playbackStep.offsetWidth;
                    this.playbackStep.style.opacity = '1';
                    this.playbackStep.style.transform = 'translateY(0)';
                }

                // Déclenche un événement pour informer les autres modules
                document.dispatchEvent(new CustomEvent('tablatureGenerated'));
            }
        })
        .catch(error => {
            console.error('Erreur lors de la génération:', error);
        })
        .finally(() => {
            // Réactive le formulaire
            if (submitButton) {
                submitButton.disabled = false;
                submitButton.innerHTML = `
                    <svg class="button-icon" viewBox="0 0 24 24">
                        <path d="M19 9l-7 7-7-7"/>
                    </svg>
                    <span>Generate Tablature</span>
                `;
            }
        });
    }

    /**
     * Nettoie les ressources utilisées par le contrôleur
     */
    cleanup() {
        if (this.generateForm) {
            this.generateForm.removeEventListener('submit', this.handleGeneration);
        }

        // Réinitialise l'état des étapes
        if (this.playbackStep) {
            this.playbackStep.classList.add('hidden');
            this.playbackStep.style.opacity = '';
            this.playbackStep.style.transform = '';
        }
    }
}


