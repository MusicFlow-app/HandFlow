import { ref, watch } from 'vue'

const THEME_KEY = 'handflow-theme'
const isDark = ref(localStorage.getItem(THEME_KEY) === 'dark')

export function useTheme() {
    const toggleTheme = () => {
        isDark.value = !isDark.value
        updateTheme()
    }

    const updateTheme = () => {
        // Update localStorage
        localStorage.setItem(THEME_KEY, isDark.value ? 'dark' : 'light')
        
        // Update data-theme attribute
        document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light')
    }

    // Initialize theme
    updateTheme()

    // Watch for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    const handleSystemThemeChange = (e) => {
        if (localStorage.getItem(THEME_KEY) === null) {
            isDark.value = e.matches
            updateTheme()
        }
    }
    mediaQuery.addEventListener('change', handleSystemThemeChange)

    return {
        isDark,
        toggleTheme
    }
}
