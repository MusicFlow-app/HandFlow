export class DisplayOptions {
    constructor() {
        this.initializeDisplayControls();
    }

    initializeDisplayControls() {
        // Initialisation des contrôles d'affichage
        this.initializeDisplayToggles();
        this.initializeSizeControls();
    }

    initializeDisplayToggles() {
        const toggles = {
            inlineDisplay: this.updateFlexDirection.bind(this),
            showSvg: this.showHandpanSVG.bind(this),
            showRestColor: this.toggleClassOnRest.bind(this),
            togglePlayInScale: this.togglePlayInScale.bind(this)
        };

        Object.entries(toggles).forEach(([id, handler]) => {
            const toggle = document.getElementById(id);
            if (toggle) {
                toggle.addEventListener('change', handler);
            }
        });
    }

    initializeSizeControls() {
        const increaseButton = document.getElementById("increase");
        const decreaseButton = document.getElementById("decrease");

        if (increaseButton) {
            increaseButton.addEventListener("click", () => this.adjustSvgSize(25, 5, true));
        }
        if (decreaseButton) {
            decreaseButton.addEventListener("click", () => this.adjustSvgSize(-25, -5, false));
        }
    }

    updateFlexDirection() {
        const measuresContainer = document.querySelector('.measures-container');
        const measures = document.querySelectorAll('.measure');
        const autoplay = document.querySelectorAll('.autoPlay');
        const toggle = document.getElementById('inlineDisplay');
        
        if (!toggle || !measuresContainer) return;
        
        const isInline = toggle.checked;
        
        autoplay.forEach(autoplaydiv => {
            autoplaydiv.style.display = isInline ? "block" : "none";
        });
        
        measuresContainer.style.flexDirection = isInline ? 'row' : 'column';
        
        measures.forEach(mesure => {
            mesure.style.flexDirection = isInline ? 'column' : 'row';
        });
    }

    showHandpanSVG() {
        const toggle = document.getElementById('showSvg');
        if (!toggle) return;

        const svgElements = document.querySelectorAll('.handpansvg svg');
        svgElements.forEach(svg => {
            svg.style.display = toggle.checked ? "initial" : "none";
        });
    }

    toggleClassOnRest() {
        const toggle = document.getElementById('showRestColor');
        if (!toggle) return;

        const elements = document.querySelectorAll('.rest-svg');
        elements.forEach(element => {
            element.classList.toggle('rest-svg-out', !toggle.checked);
        });
    }

    togglePlayInScale() {
        const toggle = document.getElementById('togglePlayInScale');
        const playOnlyInscale = document.getElementById('play_only_inscale');
        
        if (!toggle || !playOnlyInscale) return;
        
        playOnlyInscale.value = toggle.checked ? "1" : "0";
        this.regenerateDisplayIfNeeded();
    }

    adjustSvgSize(svgDelta, restDelta, increase) {
        const svgElements = document.querySelectorAll('.svg_container svg');
        let actualSize;

        svgElements.forEach(svg => {
            let currentWidth = parseFloat(window.getComputedStyle(svg).width);
            currentWidth += svg.parentElement.classList.contains('restsvg') ? restDelta : svgDelta;
            actualSize = currentWidth;
            svg.style.width = `${currentWidth}px`;
        });

        if (increase && actualSize <= 1270) {
            this.textControl('.sigN', true);
            this.textControl('.sigD', true);
        } else if (!increase && actualSize >= 150) {
            this.textControl('.sigN', false);
            this.textControl('.sigD', false);
        }
    }

    textControl(selector, increase) {
        const element = document.querySelector(selector);
        if (!element) {
            console.log(`Element with selector ${selector} not found.`);
            return;
        }

        const currentFontSize = parseFloat(window.getComputedStyle(element).fontSize);
        const currentFontSizeInEm = currentFontSize / 16;
        const newFontSizeInEm = currentFontSizeInEm + (increase ? 0.25 : -0.25);
        
        element.style.fontSize = `${newFontSizeInEm}em`;
        element.style.lineHeight = `${newFontSizeInEm / 2}em`;
    }

    regenerateDisplayIfNeeded() {
        const mesuresDiv = document.getElementById("generate-container");
        if (mesuresDiv && mesuresDiv.innerHTML.trim() !== '') {
            // Déclencher un événement personnalisé pour la régénération
            document.dispatchEvent(new CustomEvent('regenerateDisplay'));
        }
    }

    cleanup() {
        // Nettoyage des event listeners en remplaçant les éléments
        const elements = [
            'inlineDisplay', 'showSvg', 'showRestColor', 'togglePlayInScale',
            'increase', 'decrease'
        ];
        
        elements.forEach(id => {
            const element = document.getElementById(id);
            if (element) {
                element.replaceWith(element.cloneNode(true));
            }
        });
    }
}
