export class LegendModal {
    constructor() {
        this.button = document.getElementById('showLegend');
        this.modal = document.getElementById('legendModal');
        this.closeButton = this.modal.querySelector('.close-modal');
        
        this.init();
    }

    init() {
        this.button.addEventListener('click', () => this.showModal());
        this.closeButton.addEventListener('click', () => this.hideModal());
        this.modal.addEventListener('click', (e) => {
            if (e.target === this.modal) {
                this.hideModal();
            }
        });

        // Gestion des touches clavier
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && this.modal.getAttribute('aria-hidden') === 'false') {
                this.hideModal();
            }
        });
    }

    showModal() {
        this.modal.setAttribute('aria-hidden', 'false');
        this.button.setAttribute('aria-expanded', 'true');
        document.body.style.overflow = 'hidden'; // Empêche le défilement du body
    }

    hideModal() {
        this.modal.setAttribute('aria-hidden', 'true');
        this.button.setAttribute('aria-expanded', 'false');
        document.body.style.overflow = ''; // Restaure le défilement
    }
}
