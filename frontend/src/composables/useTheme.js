import { ref, watch } from 'vue'

const THEME_KEY = 'handflow-theme'
const THEME_SYSTEM = 'system'
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

// Initialize with system preference if no theme is set
const storedTheme = localStorage.getItem(THEME_KEY)
const useSystemTheme = storedTheme === THEME_SYSTEM || storedTheme === null
const isDark = ref(useSystemTheme ? mediaQuery.matches : storedTheme === 'dark')

export function useTheme() {
    const updateTheme = () => {
        const themeValue = isDark.value ? 'dark' : 'light'
        document.documentElement.setAttribute('data-theme', themeValue)
        
        // Only update localStorage if not using system theme
        const currentTheme = localStorage.getItem(THEME_KEY)
        if (currentTheme !== THEME_SYSTEM) {
            localStorage.setItem(THEME_KEY, themeValue)
        }
    }

    const toggleTheme = () => {
        isDark.value = !isDark.value
        localStorage.setItem(THEME_KEY, isDark.value ? 'dark' : 'light')
        updateTheme()
    }

    const syncWithSystem = () => {
        isDark.value = mediaQuery.matches
        localStorage.setItem(THEME_KEY, THEME_SYSTEM)
        updateTheme()
    }

    const handleSystemThemeChange = (e) => {
        if (localStorage.getItem(THEME_KEY) === THEME_SYSTEM) {
            isDark.value = e.matches
            updateTheme()
        }
    }

    // Watch for system theme changes
    mediaQuery.addEventListener('change', handleSystemThemeChange)

    // Cleanup listener on component unmount
    if (import.meta.env.SSR === false) {
        const cleanup = () => {
            mediaQuery.removeEventListener('change', handleSystemThemeChange)
        }
        
        if (typeof window !== 'undefined') {
            window.addEventListener('beforeunload', cleanup)
        }
    }

    // Initialize theme
    updateTheme()

    return {
        isDark,
        toggleTheme,
        syncWithSystem,
        isSystemTheme: () => localStorage.getItem(THEME_KEY) === THEME_SYSTEM
    }
}
