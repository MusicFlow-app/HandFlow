<script setup>
import { RouterView } from 'vue-router'
import logo from '@/assets/img/handflow-logo.svg'
</script>

<template>
  <div class="app-wrapper">
    <header class="app-header">
      <router-link to="/" class="logo-link">
        <img :src="logo" alt="HandFlow Logo" class="logo" fetchpriority="high">
        <h1 class="title">HandFlow</h1>
        <h2 class="sub-title">Handpan Tablature Generator</h2>
      </router-link>
    </header>

    <main class="app-main">
      <div class="main-content">
        <RouterView />
      </div>
    </main>

    <footer class="app-footer">
      <div class="footer-content">
        <div class="footer-links">
          <a href="https://github.com/MusicFlow-app/HandFlow" class="github-link" target="_blank" rel="noopener">
            <svg class="github-icon" viewBox="0 0 24 24" width="24" height="24">
              <path fill="currentColor" d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
            </svg>
            <span>GitHub</span>
          </a>
        </div>
        <p class="copyright">© {{ new Date().getFullYear() }} HandFlow</p>
      </div>
    </footer>
  </div>
</template>

<style>
.app-wrapper {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-header {
  padding: var(--spacing-md);
  text-align: center;
}

.logo-link {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-md);
  text-decoration: none;
  color: var(--color-text);
}

.logo {
  height: 40px;
  width: auto;
}

.title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
}

.sub-title {
  font-size: 1rem;
  font-weight: 400;
  margin: 0;
  color: var(--color-text-muted);
}

.app-main {
  flex: 1;
  padding: var(--spacing-xl) var(--spacing-md);
}

.app-footer {
  padding: var(--spacing-lg) var(--spacing-md);
}

.footer-content {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.footer-links {
  display: flex;
  gap: var(--spacing-md);
}

.github-link {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  text-decoration: none;
  color: var(--color-text);
  transition: color 0.2s;
}

.github-link:hover {
  color: var(--color-primary);
}

.github-icon {
  width: 24px;
  height: 24px;
}

.copyright {
  color: var(--color-text-muted);
  margin: 0;
}
</style>

