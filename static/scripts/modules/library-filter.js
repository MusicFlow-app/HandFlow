export class LibraryFilter {
    constructor() {
        // Clean up any existing event listeners
        if (window.libraryFilterInstance) {
            window.libraryFilterInstance.cleanup();
        }
        window.libraryFilterInstance = this;
        
        this.categoryFilter = document.getElementById('category-filter');
        this.categoryLevels = document.querySelectorAll('.category-level');
        this.tablatureCards = document.querySelectorAll('.tablature-card');
        this.difficultyLevels = document.querySelectorAll('.difficulty-level');
        
        this.currentCategory = 'all';
        this.currentDifficulty = 'all';
        
        this.init();
    }
    
    cleanup() {
        // Remove event listeners
        this.categoryLevels?.forEach(level => {
            level.removeEventListener('click', this.handleCategoryClick);
        });
        
        this.difficultyLevels?.forEach(level => {
            level.removeEventListener('click', this.handleDifficultyClick);
        });
    }
    
    init() {
        // Bind event handlers
        this.handleCategoryClick = this.handleCategoryClick.bind(this);
        this.handleDifficultyClick = this.handleDifficultyClick.bind(this);
        
        // Event listeners for category levels
        this.categoryLevels?.forEach(level => {
            level.addEventListener('click', this.handleCategoryClick);
        });
        
        // Event listeners for difficulty levels
        this.difficultyLevels?.forEach(level => {
            level.addEventListener('click', this.handleDifficultyClick);
        });
        
        // Initialiser la position des tracks
        const activeCategory = document.querySelector('.category-level.active');
        const activeDifficulty = document.querySelector('.difficulty-level.active');
        
        if (activeCategory) {
            const categorySlider = activeCategory.closest('.category-slider');
            const track = categorySlider.querySelector('.slider-track');
            const buttonRect = activeCategory.getBoundingClientRect();
            const sliderRect = categorySlider.getBoundingClientRect();
            
            track.style.width = `${buttonRect.width}px`;
            track.style.left = `${buttonRect.left - sliderRect.left}px`;
        }
    }
    
    handleCategoryClick(e) {
        const button = e.target.closest('.category-level');
        if (!button) return;
        
        this.categoryLevels.forEach(l => l.classList.remove('active'));
        button.classList.add('active');
        this.currentCategory = button.dataset.category;
        
        // Animer le slider track
        const categorySlider = button.closest('.category-slider');
        const track = categorySlider.querySelector('.slider-track');
        const buttonRect = button.getBoundingClientRect();
        const sliderRect = categorySlider.getBoundingClientRect();
        
        track.style.width = `${buttonRect.width}px`;
        track.style.left = `${buttonRect.left - sliderRect.left}px`;
        
        this.filterCards();
    }
    
    handleDifficultyClick(e) {
        this.difficultyLevels.forEach(l => l.classList.remove('active'));
        e.target.classList.add('active');
        this.currentDifficulty = e.target.dataset.difficulty;
        this.filterCards();
    }
    
    filterCards() {
        this.tablatureCards?.forEach(card => {
            const cardCategory = card.dataset.category;
            const cardDifficulty = card.dataset.difficulty;
            
            const matchesCategory = this.currentCategory === 'all' || cardCategory === this.currentCategory;
            const matchesDifficulty = this.currentDifficulty === 'all' || cardDifficulty === this.currentDifficulty;
            
            if (matchesCategory && matchesDifficulty) {
                card.style.display = '';
                card.style.opacity = '1';
                card.style.transform = 'translateY(0)';
            } else {
                card.style.display = 'none';
                card.style.opacity = '0';
                card.style.transform = 'translateY(10px)';
            }
        });
    }
}
