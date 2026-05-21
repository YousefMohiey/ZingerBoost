/**
 * ZingerBoost Application Controller
 * Modern UI/UX with improved interactions and accessibility
 */

const AppState = {
    currentTab: 'home',
    optCategory: 'all',
    svcCategory: 'all',
    debloatCategory: 'all',
    installerCategory: 'all',
    gameModeActive: false,
    metricsInterval: null,
    tweaksData: [],
    servicesData: [],
    cleanerData: [],
    bloatwareData: [],
    softwareData: [],
    statusTimeout: null,
    favorites: new Set(),
    isLoading: false,
    // Window state tracking
    isMaximized: false
};

const DOM = {
    sidebarNav: null,
    statusBar: null,
    statusText: null,
    statusClose: null,
    pageTitle: null,
    topbarActions: null,
    tabs: {},
    lists: {}
};

const tabTitles = {
    home: 'Home',
    optimizations: 'Optimizations',
    services: 'Services',
    cleaner: 'Cleaner',
    debloat: 'Debloat',
    installer: 'Installer',
    backups: 'Backups',
    gamemode: 'Game Mode',
    favorites: 'Favorites',
    audit: 'Audit Log',
    settings: 'Settings'
};

const catLabels = {
    visual: 'Visual',
    privacy: 'Privacy',
    performance: 'Performance',
    gaming: 'Gaming',
    network: 'Network',
    windows_update: 'Windows Update'
};

/**
 * Initialize application
 */
document.addEventListener('DOMContentLoaded', async () => {
    cacheDOM();
    Modal.init();
    setupEventListeners();
    loadAppInfo();
    checkAdminStatus();
    loadInitialData();
    await startMetricsPolling();
});

/**
 * Cache DOM references
 */
function cacheDOM() {
    DOM.sidebarNav = document.getElementById('sidebar-nav');
    DOM.statusBar = document.getElementById('status-bar');
    DOM.statusText = document.getElementById('status-text');
    DOM.statusClose = document.getElementById('status-close');
    DOM.pageTitle = document.getElementById('page-title');
    DOM.topbarActions = document.getElementById('topbar-actions');

    // Titlebar elements
    DOM.titlebar = document.getElementById('titlebar');
    DOM.titlebarMinimize = document.getElementById('titlebar-minimize');
    DOM.titlebarMaximize = document.getElementById('titlebar-maximize');
    DOM.titlebarClose = document.getElementById('titlebar-close');
    DOM.iconMaximize = document.getElementById('icon-maximize');
    DOM.iconRestore = document.getElementById('icon-restore');

    document.querySelectorAll('.tab-content').forEach(tab => {
        DOM.tabs[tab.id.replace('tab-', '')] = tab;
    });

    DOM.lists = {
        tweaks: document.getElementById('tweaks-list'),
        services: document.getElementById('services-list'),
        cleaner: document.getElementById('cleaner-list'),
        backups: document.getElementById('backups-list'),
        bloatware: document.getElementById('bloatware-list'),
        software: document.getElementById('software-list'),
        favorites: document.getElementById('favorites-list')
    };
}

/**
 * Setup event listeners
 */
function setupEventListeners() {
    // Sidebar navigation
    DOM.sidebarNav.querySelectorAll('.nav-btn').forEach(btn => {
        btn.addEventListener('click', () => switchTab(btn.dataset.tab));
    });

    // Status bar close
    DOM.statusClose.addEventListener('click', hideStatus);

    // Quick action cards
    document.querySelectorAll('.action-card[data-tab]').forEach(card => {
        card.addEventListener('click', () => switchTab(card.dataset.tab));
    });

    // Home Clean All button
    const homeCleanAllBtn = document.getElementById('home-clean-all-btn');
    if (homeCleanAllBtn) homeCleanAllBtn.addEventListener('click', handleCleanAll);

    // Optimization category tabs
    const optTabs = document.getElementById('opt-tabs');
    if (optTabs) {
        optTabs.querySelectorAll('.opt-tab').forEach(btn => {
            btn.addEventListener('click', () => {
                optTabs.querySelectorAll('.opt-tab').forEach(b => {
                    b.classList.remove('active');
                    b.setAttribute('aria-selected', 'false');
                });
                btn.classList.add('active');
                btn.setAttribute('aria-selected', 'true');
                AppState.optCategory = btn.dataset.cat;
                renderTweaks();
            });
        });
    }

    // Services category tabs
    const svcTabs = document.getElementById('svc-tabs');
    if (svcTabs) {
        svcTabs.querySelectorAll('.opt-tab').forEach(btn => {
            btn.addEventListener('click', () => {
                svcTabs.querySelectorAll('.opt-tab').forEach(b => {
                    b.classList.remove('active');
                    b.setAttribute('aria-selected', 'false');
                });
                btn.classList.add('active');
                btn.setAttribute('aria-selected', 'true');
                AppState.svcCategory = btn.dataset.cat;
                renderServices();
            });
        });
    }

    // Debloat category tabs
    const debloatTabs = document.getElementById('debloat-tabs');
    if (debloatTabs) {
        debloatTabs.querySelectorAll('.opt-tab').forEach(btn => {
            btn.addEventListener('click', () => {
                debloatTabs.querySelectorAll('.opt-tab').forEach(b => {
                    b.classList.remove('active');
                    b.setAttribute('aria-selected', 'false');
                });
                btn.classList.add('active');
                btn.setAttribute('aria-selected', 'true');
                AppState.debloatCategory = btn.dataset.cat;
                renderBloatware();
            });
        });
    }

    // Installer category tabs
    const installerTabs = document.getElementById('installer-tabs');
    if (installerTabs) {
        installerTabs.querySelectorAll('.opt-tab').forEach(btn => {
            btn.addEventListener('click', () => {
                installerTabs.querySelectorAll('.opt-tab').forEach(b => {
                    b.classList.remove('active');
                    b.setAttribute('aria-selected', 'false');
                });
                btn.classList.add('active');
                btn.setAttribute('aria-selected', 'true');
                AppState.installerCategory = btn.dataset.cat;
                renderSoftware();
            });
        });
    }

    // Game mode toggle
    const gmToggle = document.getElementById('gamemode-toggle');
    if (gmToggle) gmToggle.addEventListener('change', handleGameModeToggle);

    // Uninstall button
    const uninstallBtn = document.getElementById('uninstall-btn');
    if (uninstallBtn) uninstallBtn.addEventListener('click', handleUninstall);

    // Check update button
    const checkUpdateBtn = document.getElementById('check-update-btn');
    if (checkUpdateBtn) checkUpdateBtn.addEventListener('click', handleCheckUpdate);

    // Delegated event listeners for dynamic content
    document.addEventListener('change', (e) => {
        const toggle = e.target;
        if (!toggle || !toggle.classList) return;
        
        if (toggle.classList.contains('tweak-toggle')) {
            const { index, id, name } = toggle.dataset;
            if (index && id && name) window.toggleTweak(index, id, name);
        } else if (toggle.classList.contains('service-toggle')) {
            const { index, name, display } = toggle.dataset;
            if (index && name) window.toggleService(index, name, display || name);
        }
    });

    document.addEventListener('click', (e) => {
        const btn = e.target.closest('button');
        if (!btn || btn.disabled) return;
        
        if (btn.classList.contains('clean-btn')) {
            const { id, name } = btn.dataset;
            if (id && name) window.cleanCategory(id, name);
        } else if (btn.classList.contains('restore-btn')) {
            const { id } = btn.dataset;
            if (id) window.restoreBackup(id);
        } else if (btn.classList.contains('delete-btn')) {
            const { id } = btn.dataset;
            if (id) window.deleteBackup(id);
        } else if (btn.classList.contains('remove-bloat-btn')) {
            const { winget, name, id } = btn.dataset;
            // Pass winget_id to backend (not internal ID like "bloat_candycrush")
            const removalId = (winget !== undefined) ? winget : (id || '');
            if (removalId !== undefined) window.removeBloatware(removalId, name || removalId);
        } else if (btn.classList.contains('install-btn')) {
            const { name, winget } = btn.dataset;
            if (name && winget) window.installSoftware(name, winget);
        } else if (btn.classList.contains('fav-btn')) {
            const { type, index } = btn.dataset;
            if (type && index) toggleFavorite(type, index);
        }
    });

    // Keyboard navigation support
    document.addEventListener('keydown', (e) => {
        // Escape: hide status bar
        if (e.key === 'Escape') {
            hideStatus();
        }

        // Arrow key navigation for sidebar tabs
        if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
            const focused = document.activeElement;
            if (focused && focused.classList.contains('nav-btn')) {
                e.preventDefault();
                const navButtons = Array.from(DOM.sidebarNav.querySelectorAll('.nav-btn'));
                const currentIndex = navButtons.indexOf(focused);
                let nextIndex;

                if (e.key === 'ArrowDown') {
                    nextIndex = (currentIndex + 1) % navButtons.length;
                } else {
                    nextIndex = (currentIndex - 1 + navButtons.length) % navButtons.length;
                }

                navButtons[nextIndex].focus();
            }
        }

        // Enter/Space on action cards triggers click
        if ((e.key === 'Enter' || e.key === ' ') && document.activeElement) {
            const focused = document.activeElement;
            if (focused.classList.contains('action-card') && focused.dataset.tab) {
                e.preventDefault();
                switchTab(focused.dataset.tab);
            }
        }
    });

    // Native app behavior: disable right-click context menu
    document.addEventListener('contextmenu', (e) => e.preventDefault());

    // Disable pinch-zoom via keyboard
    document.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && (e.key === '+' || e.key === '-' || e.key === '0')) {
            e.preventDefault();
        }
    });

    // Disable drag-and-drop of images/content
    document.addEventListener('dragstart', (e) => e.preventDefault());

    // Titlebar window control buttons
    if (DOM.titlebarMinimize) {
        DOM.titlebarMinimize.addEventListener('click', async () => {
            try {
                await window.__TAURI__.window.getCurrentWindow().minimize();
            } catch (err) {
                console.error('[titlebar] Minimize failed:', err);
            }
        });
    }

    if (DOM.titlebarMaximize) {
        DOM.titlebarMaximize.addEventListener('click', async () => {
            try {
                const win = window.__TAURI__.window.getCurrentWindow();
                await win.toggleMaximize();
            } catch (err) {
                console.error('[titlebar] ToggleMaximize failed:', err);
            }
        });
    }

    if (DOM.titlebarClose) {
        DOM.titlebarClose.addEventListener('click', async () => {
            try {
                await window.__TAURI__.window.getCurrentWindow().close();
            } catch (err) {
                console.error('[titlebar] Close failed:', err);
            }
        });
    }

    // Track window maximize/restore state via resize events
    if (window.__TAURI__ && window.__TAURI__.event) {
        window.__TAURI__.event.listen('tauri://resize', async () => {
            try {
                const win = window.__TAURI__.window.getCurrentWindow();
                const isMax = await win.isMaximized();
                updateMaximizeIcon(isMax);
            } catch (err) {
                console.error('[titlebar] Resize event failed:', err);
            }
        });

        // Check initial state on load
        (async () => {
            try {
                const win = window.__TAURI__.window.getCurrentWindow();
                const isMax = await win.isMaximized();
                updateMaximizeIcon(isMax);
            } catch (err) {
                updateMaximizeIcon(false);
            }
        })();
    }
}

/**
 * Switch between tabs
 */
function switchTab(tab) {
    AppState.currentTab = tab;
    
    // Update sidebar navigation
    DOM.sidebarNav.querySelectorAll('.nav-btn').forEach(btn => {
        const isActive = btn.dataset.tab === tab;
        btn.classList.toggle('active', isActive);
        btn.setAttribute('aria-current', isActive ? 'page' : 'false');
    });

    // Update tab content visibility
    Object.entries(DOM.tabs).forEach(([key, el]) => {
        el.classList.toggle('active', key === tab);
    });

    // Update page title
    DOM.pageTitle.textContent = tabTitles[tab] || tab;
    
    // Update topbar actions
    updateTopbarActions(tab);
    
    // Load tab-specific data
    loadTabData(tab);
}

/**
 * Update topbar action buttons based on current tab
 */
function updateTopbarActions(tab) {
    DOM.topbarActions.innerHTML = '';
    
    const actions = {
        optimizations: `
            <button class="btn-primary" id="apply-all-btn">Apply All</button>
            <button class="btn-secondary" id="revert-all-btn">Revert All</button>
        `,
        services: `
            <button class="btn-secondary" id="refresh-services-btn">Refresh</button>
        `,
        cleaner: `
            <button class="btn-secondary" id="scan-all-btn">Rescan</button>
            <button class="btn-primary" id="clean-all-btn">Clean All</button>
        `,
        backups: `
            <button class="btn-primary" id="create-backup-btn">Create Backup</button>
            <button class="btn-danger" id="clear-backups-btn">Clear All</button>
        `,
        audit: `
            <button class="btn-secondary" id="refresh-audit">Refresh</button>
            <button class="btn-danger" id="clear-audit">Clear</button>
        `
    };

    if (actions[tab]) {
        DOM.topbarActions.innerHTML = actions[tab];
        bindTopbarActions(tab);
    }
}

/**
 * Bind event listeners to topbar action buttons
 */
function bindTopbarActions(tab) {
    const applyAllBtn = document.getElementById('apply-all-btn');
    if (applyAllBtn) applyAllBtn.addEventListener('click', handleApplyAllTweaks);
    
    const revertAllBtn = document.getElementById('revert-all-btn');
    if (revertAllBtn) revertAllBtn.addEventListener('click', handleRevertAllTweaks);
    
    const scanAllBtn = document.getElementById('scan-all-btn');
    if (scanAllBtn) scanAllBtn.addEventListener('click', () => loadCleaner());
    
    const refreshServicesBtn = document.getElementById('refresh-services-btn');
    if (refreshServicesBtn) refreshServicesBtn.addEventListener('click', () => loadServices());
    
    const cleanAllBtn = document.getElementById('clean-all-btn');
    if (cleanAllBtn) cleanAllBtn.addEventListener('click', handleCleanAll);
    
    const createBackupBtn = document.getElementById('create-backup-btn');
    if (createBackupBtn) createBackupBtn.addEventListener('click', handleCreateBackup);
    
    const clearBackupsBtn = document.getElementById('clear-backups-btn');
    if (clearBackupsBtn) clearBackupsBtn.addEventListener('click', () => window.clearBackups());
    
    const refreshAuditBtn = document.getElementById('refresh-audit');
    if (refreshAuditBtn) refreshAuditBtn.addEventListener('click', () => loadAuditLog());
    
    const clearAuditBtn = document.getElementById('clear-audit');
    if (clearAuditBtn) clearAuditBtn.addEventListener('click', handleClearAudit);
}

/**
 * Load app version and info
 */
async function loadAppInfo() {
    try {
        const info = await invoke('get_app_info');
        const versionEl = document.getElementById('app-version');
        if (versionEl && info.version) versionEl.textContent = `v${info.version}`;
    } catch (err) {
        console.error('[loadAppInfo] Failed:', err);
    }
}

/**
 * Check and display admin status
 */
async function checkAdminStatus() {
    try {
        const isAdmin = await invoke('check_admin');
        const dot = document.getElementById('admin-indicator');
        const text = document.getElementById('admin-text');
        
        if (dot) {
            dot.classList.toggle('active', isAdmin);
            dot.title = isAdmin ? 'Running as Administrator' : 'Not running as Administrator';
        }
        
        if (text) {
            text.textContent = isAdmin ? 'Administrator' : 'Standard User';
        }
    } catch (err) {
        console.error('Failed to check admin status:', err);
    }
}

/**
 * Show status notification
 */
function showStatus(message, type = 'info') {
    DOM.statusText.textContent = message;
    DOM.statusBar.classList.remove('hidden');
    
    clearTimeout(AppState.statusTimeout);
    AppState.statusTimeout = setTimeout(hideStatus, 5000);
}

/**
 * Custom Modal System (replaces confirm/prompt)
 */
const Modal = {
    el: null,
    titleEl: null,
    messageEl: null,
    inputEl: null,
    cancelBtn: null,
    confirmBtn: null,
    resolver: null,

    init() {
        this.el = document.getElementById('custom-modal');
        this.titleEl = document.getElementById('modal-title');
        this.messageEl = document.getElementById('modal-message');
        this.inputEl = document.getElementById('modal-input');
        this.cancelBtn = document.getElementById('modal-cancel');
        this.confirmBtn = document.getElementById('modal-confirm');

        this.cancelBtn.addEventListener('click', () => this._resolve(false));
        this.confirmBtn.addEventListener('click', () => this._resolve(true));

        // Close on backdrop click
        this.el.querySelector('.modal-backdrop').addEventListener('click', () => this._resolve(false));

        // Close on Escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && !this.el.classList.contains('hidden')) {
                this._resolve(false);
            }
        });
    },

    show(options = {}) {
        const { title = 'Confirm', message, input = false, inputValue = '', danger = false } = options;
        return new Promise((resolve) => {
            this.resolver = resolve;
            this.titleEl.textContent = title;
            this.messageEl.textContent = message;

            if (input) {
                this.inputEl.classList.remove('hidden');
                this.inputEl.value = inputValue;
            } else {
                this.inputEl.classList.add('hidden');
            }

            // Danger styling for confirm button
            this.confirmBtn.classList.toggle('danger', danger);
            this.confirmBtn.textContent = danger ? 'Delete' : 'OK';

            this.el.classList.remove('hidden');

            // Focus the appropriate element
            if (input) {
                this.inputEl.focus();
                this.inputEl.select();
            } else {
                this.confirmBtn.focus();
            }
        });
    },

    _resolve(confirmed) {
        this.el.classList.add('hidden');
        if (this.resolver) {
            const inputValue = this.inputEl.value;
            this.resolver(confirmed ? (this.inputEl.classList.contains('hidden') ? true : inputValue) : false);
            this.resolver = null;
        }
    }
};

/**
 * Hide status notification
 */
function hideStatus() {
    DOM.statusBar.classList.add('hidden');
}

/**
 * Show progress bar
 */
function showProgress(percent) {
    const progressBar = document.getElementById('status-progress');
    const progressBarFill = document.getElementById('status-progress-bar');
    
    if (progressBar && progressBarFill) {
        progressBar.classList.remove('hidden');
        progressBarFill.style.width = `${percent}%`;
        progressBar.setAttribute('aria-valuenow', percent);
    }
}

/**
 * Hide progress bar
 */
function hideProgress() {
    const progressBar = document.getElementById('status-progress');
    if (progressBar) {
        progressBar.classList.add('hidden');
        progressBar.setAttribute('aria-valuenow', '0');
    }
}

/**
 * Handle game mode toggle
 */
async function handleGameModeToggle(e) {
    AppState.gameModeActive = e.target.checked;
    
    try {
        await invoke('toggle_game_mode', { active: AppState.gameModeActive });
        updateGameModeUI();
        showStatus(AppState.gameModeActive ? 'Game Mode activated' : 'Game Mode deactivated');
    } catch (err) {
        showStatus(`Error: ${err}`);
        e.target.checked = !AppState.gameModeActive;
    }
}

/**
 * Update game mode UI state
 */
function updateGameModeUI() {
    const toggle = document.getElementById('gamemode-toggle');
    const state = document.getElementById('gamemode-state');
    
    if (!toggle || !state) return;
    
    toggle.checked = AppState.gameModeActive;
    toggle.setAttribute('aria-checked', AppState.gameModeActive);
    
    if (AppState.gameModeActive) {
        state.textContent = 'Active';
        state.className = 'state-text active';
    } else {
        state.textContent = 'Inactive';
        state.className = 'state-text inactive';
    }
}

/**
 * Invoke Tauri backend command
 */
async function invoke(cmd, args = {}) {
    try {
        const result = await window.__TAURI__.core.invoke(cmd, args);
        
        if (typeof result === 'string') {
            const trimmed = result.trim();
            if ((trimmed.startsWith('{') && trimmed.endsWith('}')) || 
                (trimmed.startsWith('[') && trimmed.endsWith(']'))) {
                return JSON.parse(result);
            }
            return result;
        }
        
        return result;
    } catch (err) {
        console.error(`[invoke] ${cmd} FAILED:`, err);
        throw err;
    }
}

/**
 * Load initial data for all tabs
 */
async function loadInitialData() {
    await Promise.allSettled([
        loadMetrics(),
        loadTweaks(),
        loadServices(),
        loadCleaner(),
        loadBackups(),
        loadBloatware(),
        loadSoftware(),
        loadFavorites()
    ]);
}

/**
 * Load data for specific tab
 */
async function loadTabData(tab) {
    const loaders = {
        home: loadMetrics,
        optimizations: loadTweaks,
        services: loadServices,
        cleaner: loadCleaner,
        backups: loadBackups,
        debloat: loadBloatware,
        installer: loadSoftware,
        favorites: renderFavorites,
        audit: loadAuditLog
    };
    
    if (loaders[tab]) {
        await loaders[tab]();
    }
}

/**
 * Load system metrics
 */
async function loadMetrics() {
    try {
        const metrics = await invoke('get_metrics');
        updateHomeMetrics(metrics);
        
        const tsEl = document.getElementById('metrics-last-update');
        if (tsEl) {
            const now = new Date();
            tsEl.textContent = now.toLocaleTimeString();
        }
        
        return metrics;
    } catch (err) {
        console.error('[metrics] Failed to load:', err);
        return null;
    }
}

/**
 * Update home page metrics
 */
function updateHomeMetrics(metrics) {
    if (!metrics) return;
    
    const cpuPct = metrics.cpu_percent;
    const ramPct = metrics.ram_percent;
    const diskPct = metrics.disk_active_percent;
    const netMbps = metrics.network_down_mbps + metrics.network_up_mbps;
    
    const cpuVal = document.getElementById('home-cpu-value');
    const cpuBar = document.getElementById('home-cpu-bar');
    if (cpuVal) { cpuVal.textContent = `${Math.round(cpuPct)}%`; flashElement(cpuVal); }
    if (cpuBar) { cpuBar.style.width = `${Math.min(cpuPct, 100)}%`; cpuBar.parentElement.setAttribute('aria-valuenow', Math.round(cpuPct)); }
    
    const ramVal = document.getElementById('home-ram-value');
    const ramBar = document.getElementById('home-ram-bar');
    if (ramVal) { ramVal.textContent = `${Math.round(ramPct)}%`; flashElement(ramVal); }
    if (ramBar) { ramBar.style.width = `${Math.min(ramPct, 100)}%`; ramBar.parentElement.setAttribute('aria-valuenow', Math.round(ramPct)); }
    
    const diskVal = document.getElementById('home-disk-value');
    const diskBar = document.getElementById('home-disk-bar');
    if (diskVal) { diskVal.textContent = `${Math.round(diskPct)}%`; flashElement(diskVal); }
    if (diskBar) { diskBar.style.width = `${Math.min(diskPct, 100)}%`; diskBar.parentElement.setAttribute('aria-valuenow', Math.round(diskPct)); }
    
    const netVal = document.getElementById('home-network-value');
    const netBar = document.getElementById('home-network-bar');
    if (netVal) { netVal.textContent = netMbps < 1 ? `${(netMbps * 1000).toFixed(0)} Kbps` : `${netMbps.toFixed(1)} Mbps`; flashElement(netVal); }
    if (netBar) { const np = Math.min(netMbps * 5, 100); netBar.style.width = `${np}%`; netBar.parentElement.setAttribute('aria-valuenow', Math.round(np)); }
}

function flashElement(el) {
    if (!el) return;
    el.classList.add('metric-updated');
    clearTimeout(el._flashTimeout);
    el._flashTimeout = setTimeout(() => el.classList.remove('metric-updated'), 300);
}

/**
 * Start metrics updates. Backend pushes via eval() every second.
 * This function just does initial load.
 */
async function startMetricsPolling() {
    // Initial load
    await loadMetrics();
}

/**
 * Render list with items
 */
function renderList(container, items, renderItem) {
    if (!container) return;
    
    if (!items || items.length === 0) {
        container.innerHTML = '<p class="empty-state">No items available</p>';
        return;
    }
    
    container.innerHTML = items.map(renderItem).join('');
}

/**
 * Show loading state
 */
function showLoading(container) {
    if (!container) return;
    container.innerHTML = `
        <div class="loading-spinner">
            <div class="spinner"></div>
            <p>Loading...</p>
        </div>
    `;
}

/**
 * Show error state
 */
function showError(container, message) {
    if (!container) return;
    container.innerHTML = `<p class="empty-state error">${escapeHtml(message)}</p>`;
}

/**
 * Get CSS class for category
 */
function getCatClass(category) {
    const map = {
        'Visual': 'cat-visual',
        'Privacy': 'cat-privacy',
        'Performance': 'cat-performance',
        'Gaming': 'cat-gaming',
        'Network': 'cat-network',
        'Windows Update': 'cat-windows',
        'Service': 'cat-service',
        'Cleaner': 'cat-cleaner',
        'Debloat': 'cat-debloat',
        'Backup': 'cat-backup'
    };
    return map[category] || '';
}

/**
 * Render a card component
 */
function renderCard({
    title,
    description,
    badge,
    statusText,
    statusActive,
    hasToggle,
    toggleChecked,
    toggleClass,
    toggleData,
    hasButton,
    buttonText,
    buttonClass,
    buttonData,
    buttonDisabled,
    favType,
    favIndex,
    isFav
}) {
    const catClass = getCatClass(badge);
    
    return `
        <div class="card" role="listitem">
            <div class="card-header">
                <div class="card-info">
                    <div class="card-badge ${catClass}">${escapeHtml(badge)}</div>
                    <h3>${escapeHtml(title)}</h3>
                    <p>${escapeHtml(description)}</p>
                </div>
                <button class="fav-btn ${isFav ? 'active' : ''}" 
                        data-type="${escapeHtml(favType)}" 
                        data-index="${favIndex}" 
                        title="${isFav ? 'Remove from favorites' : 'Add to favorites'}"
                        aria-label="${isFav ? 'Remove from favorites' : 'Add to favorites'}">
                    <svg viewBox="0 0 24 24" fill="${isFav ? 'currentColor' : 'none'}" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
                    </svg>
                </button>
            </div>
            <div class="card-footer">
                <div class="card-status ${statusActive ? 'active' : 'inactive'}">
                    <span class="status-dot"></span>
                    <span>${escapeHtml(statusText)}</span>
                </div>
                ${hasToggle ? `
                    <label class="toggle-switch">
                        <input type="checkbox" 
                               class="${toggleClass}" 
                               ${toggleData} 
                               ${toggleChecked ? 'checked' : ''}
                               role="switch"
                               aria-checked="${toggleChecked}">
                        <span class="toggle-slider"></span>
                    </label>
                ` : ''}
                ${hasButton ? `
                    <button class="btn-action ${buttonClass || ''}" 
                            ${buttonData} 
                            ${buttonDisabled ? 'disabled' : ''}>
                        ${escapeHtml(buttonText)}
                    </button>
                ` : ''}
            </div>
        </div>
    `;
}

/**
 * Load tweaks data
 */
async function loadTweaks() {
    showLoading(DOM.lists.tweaks);
    
    try {
        const data = await invoke('get_tweaks');
        AppState.tweaksData = Array.isArray(data) ? data : [];
        renderTweaks();
        
        const countEl = document.getElementById('home-tweak-count');
        if (countEl) countEl.textContent = AppState.tweaksData.length;
    } catch (err) {
        showError(DOM.lists.tweaks, 'Failed to load tweaks: ' + err);
    }
}

/**
 * Render tweaks list
 */
function renderTweaks() {
    if (!AppState.tweaksData.length) {
        renderList(DOM.lists.tweaks, [], () => '');
        return;
    }
    
    invoke('get_tweak_states').then(states => {
        const stateMap = new Map(states || []);
        const filtered = AppState.optCategory === 'all' 
            ? AppState.tweaksData 
            : AppState.tweaksData.filter(t => t.category === AppState.optCategory);
        
        renderList(DOM.lists.tweaks, filtered, (tweak) => {
            const i = AppState.tweaksData.indexOf(tweak);
            const applied = stateMap.get(tweak.id) || false;
            const isFav = AppState.favorites.has(`tweak-${tweak.id}`);
            
            return renderCard({
                title: tweak.name,
                description: tweak.description,
                badge: catLabels[tweak.category] || tweak.category,
                statusText: applied ? 'Active' : 'Inactive',
                statusActive: applied,
                hasToggle: true,
                toggleChecked: applied,
                toggleClass: 'tweak-toggle',
                toggleData: `data-index="${i}" data-id="${escapeHtml(tweak.id)}" data-name="${escapeHtml(tweak.name)}"`,
                favType: 'tweak',
                favIndex: tweak.id,
                isFav
            });
        });
    }).catch(err => {
        console.error('[renderTweaks] Failed to get states:', err);
        // Render without states if get_tweak_states fails
        const stateMap = new Map();
        const filtered = AppState.optCategory === 'all' 
            ? AppState.tweaksData 
            : AppState.tweaksData.filter(t => t.category === AppState.optCategory);
        
        renderList(DOM.lists.tweaks, filtered, (tweak) => {
            const i = AppState.tweaksData.indexOf(tweak);
            const applied = stateMap.get(tweak.id) || false;
            const isFav = AppState.favorites.has(`tweak-${tweak.id}`);
            
            return renderCard({
                title: tweak.name,
                description: tweak.description,
                badge: catLabels[tweak.category] || tweak.category,
                statusText: applied ? 'Active' : 'Inactive',
                statusActive: applied,
                hasToggle: true,
                toggleChecked: applied,
                toggleClass: 'tweak-toggle',
                toggleData: `data-index="${i}" data-id="${escapeHtml(tweak.id)}" data-name="${escapeHtml(tweak.name)}"`,
                favType: 'tweak',
                favIndex: tweak.id,
                isFav
            });
        });
    });
}

/**
 * Toggle a tweak
 */
window.toggleTweak = async function(index, id, name) {
    const toggle = document.querySelector(`.tweak-toggle[data-index="${index}"]`);
    if (!toggle) return;
    
    const isActive = toggle.checked;
    showStatus(isActive ? `Applying ${name}...` : `Reverting ${name}...`);
    toggle.disabled = true;
    
    try {
        const cmd = isActive ? 'apply_tweak' : 'revert_tweak';
        const result = await invoke(cmd, { id });
        
        const card = toggle.closest('.card');
        if (card) {
            const status = card.querySelector('.card-status');
            if (status) {
                status.className = `card-status ${isActive ? 'active' : ''}`;
                status.innerHTML = `
                    <span class="status-dot"></span>
                    <span>${isActive ? 'Active' : 'Inactive'}</span>
                `;
            }
        }
        
        showStatus(result.message || `${isActive ? 'Applied' : 'Reverted'}: ${name}`);
    } catch (err) {
        showStatus(`Error: ${err}`);
        toggle.checked = !isActive;
    } finally {
        toggle.disabled = false;
    }
};

/**
 * Apply all tweaks
 */
async function handleApplyAllTweaks() {
    const confirmed = await Modal.show({ title: 'Apply All', message: 'Apply all optimizations? This will modify system settings.' });
    if (!confirmed) return;
    
    showStatus('Applying all optimizations...');
    
    try {
        const result = await invoke('apply_all_tweaks');
        showStatus(result.message || result);
        await loadTweaks();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
}

/**
 * Revert all tweaks
 */
async function handleRevertAllTweaks() {
    const confirmed = await Modal.show({ title: 'Revert All', message: 'Revert all optimizations? This will restore original settings.' });
    if (!confirmed) return;
    
    showStatus('Reverting all optimizations...');
    
    try {
        const result = await invoke('revert_all_tweaks');
        showStatus(result.message || result);
        await loadTweaks();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
}

/**
 * Load services data
 */
async function loadServices() {
    showLoading(DOM.lists.services);
    
    try {
        AppState.servicesData = await invoke('get_services');
        renderServices();
    } catch (err) {
        showError(DOM.lists.services, 'Failed to load services');
    }
}

/**
 * Render services list with filtering
 */
function renderServices() {
    if (!AppState.servicesData.length) {
        renderList(DOM.lists.services, [], () => '');
        return;
    }
    
    const filtered = AppState.svcCategory === 'all' 
        ? AppState.servicesData 
        : AppState.servicesData.filter(svc => {
            if (AppState.svcCategory === 'running') return svc.status === 'Running';
            if (AppState.svcCategory === 'stopped') return svc.status !== 'Running';
            return true;
        });
    
    renderList(DOM.lists.services, filtered, (svc, i) => {
        const isRunning = svc.status === 'Running';
        const isFav = AppState.favorites.has(`service-${svc.name}`);
        
        return renderCard({
            title: svc.display_name,
            description: isRunning ? 'Currently running' : 'Currently stopped',
            badge: 'Service',
            statusText: svc.status,
            statusActive: isRunning,
            hasToggle: true,
            toggleChecked: isRunning,
            toggleClass: 'service-toggle',
            toggleData: `data-index="${i}" data-name="${escapeHtml(svc.name)}" data-display="${escapeHtml(svc.display_name)}"`,
            favType: 'service',
            favIndex: svc.name,
            isFav
        });
    });
}

/**
 * Toggle a service
 */
window.toggleService = async function(index, name, displayName) {
    const toggle = document.querySelector(`.service-toggle[data-index="${index}"]`);
    if (!toggle) return;
    
    const shouldBeActive = toggle.checked;
    showStatus(shouldBeActive ? `Starting ${displayName}...` : `Stopping ${displayName}...`);
    toggle.disabled = true;
    
    try {
        if (shouldBeActive) {
            await invoke('start_service', { name });
            showStatus(`Started: ${displayName}`);
        } else {
            await invoke('disable_service', { name });
            showStatus(`Stopped: ${displayName}`);
        }
        
        // Auto-refresh services list after toggle
        await loadServices();
    } catch (err) {
        showStatus(`Error: ${err}`);
        toggle.checked = !shouldBeActive;
    } finally {
        toggle.disabled = false;
    }
};

/**
 * Load cleaner data
 */
async function loadCleaner() {
    showLoading(DOM.lists.cleaner);
    
    try {
        const items = await invoke('get_cleaner_items');
        AppState.cleanerData = Array.isArray(items) ? items : [];
        
        renderList(DOM.lists.cleaner, items, (item, i) => renderCard({
            title: item.name,
            description: `${item.size_mb.toFixed(1)} MB reclaimable`,
            badge: 'Cleaner',
            statusText: item.risk,
            statusActive: item.risk === 'safe',
            hasButton: true,
            buttonText: 'Clean',
            buttonData: `data-id="${escapeHtml(item.id)}" data-name="${escapeHtml(item.name)}" data-winget="${escapeHtml(item.winget_id)}"`,
            buttonClass: 'clean-btn',
            favType: 'cleaner',
            favIndex: item.id,
            isFav: AppState.favorites.has(`cleaner-${item.id}`)
        }));
    } catch (err) {
        showError(DOM.lists.cleaner, 'Failed to load cleaner items');
    }
}

/**
 * Clean a category
 */
window.cleanCategory = async function(id, name) {
    showStatus(`Cleaning ${name}...`);
    
    try {
        const result = await invoke('clean_category', { name: id });
        showStatus(result);
        await loadCleaner();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
};

/**
 * Clean all categories
 */
async function handleCleanAll() {
    const confirmed = await Modal.show({ title: 'Clean All', message: 'Clean all categories? This will delete temporary files, caches, and logs.' });
    if (!confirmed) return;
    
    showStatus('Cleaning all categories...');
    
    try {
        const result = await invoke('clean_all');
        showStatus(result);
        await loadCleaner();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
}

/**
 * Load backups data
 */
async function loadBackups() {
    showLoading(DOM.lists.backups);
    
    try {
        const backups = await invoke('get_backups');
        
        const container = DOM.lists.backups;
        if (!container) return;
        
        if (!backups || backups.length === 0) {
            container.innerHTML = '<p class="empty-state">No backups available</p>';
            const countEl = document.getElementById('home-backup-count');
            if (countEl) countEl.textContent = '0';
            return;
        }
        
        container.innerHTML = backups.map((backup, i) => `
            <div class="card" role="listitem">
                <div class="card-header">
                    <div class="card-info">
                        <div class="card-badge cat-backup">Backup</div>
                        <h3>${escapeHtml(backup.id.slice(0, 8))}</h3>
                        <p>${backup.tweak_count} tweak(s) — ${escapeHtml(backup.description)}</p>
                    </div>
                </div>
                <div class="card-footer">
                    <div class="card-status inactive">
                        <span class="status-dot"></span>
                        <span>Ready</span>
                    </div>
                    <div class="backup-actions">
                        <button class="btn-action restore-btn" data-id="${escapeHtml(backup.id)}">Restore</button>
                        <button class="btn-action delete-btn" data-id="${escapeHtml(backup.id)}">Delete</button>
                    </div>
                </div>
            </div>
        `).join('');
        
        const countEl = document.getElementById('home-backup-count');
        if (countEl) countEl.textContent = backups.length;
    } catch (err) {
        showError(DOM.lists.backups, 'Failed to load backups');
    }
}

/**
 * Restore a backup
 */
window.restoreBackup = async function(id) {
    showStatus(`Restoring backup ${id}...`);
    
    try {
        const result = await invoke('restore_backup', { id });
        showStatus(result);
        await loadBackups();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
};

/**
 * Delete a backup
 */
window.deleteBackup = async function(id) {
    const confirmed = await Modal.show({ title: 'Delete Backup', message: 'Delete this backup? This cannot be undone.', danger: true });
    if (!confirmed) return;
    
    showStatus(`Deleting backup ${id}...`);
    
    try {
        const result = await invoke('delete_backup', { id });
        showStatus(result);
        await loadBackups();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
};

/**
 * Clear all backups
 */
window.clearBackups = async function() {
    const confirmed = await Modal.show({ title: 'Clear All Backups', message: 'Delete ALL backups? This cannot be undone.', danger: true });
    if (!confirmed) return;
    
    showStatus('Clearing all backups...');
    
    try {
        const result = await invoke('clear_backups');
        showStatus(result);
        await loadBackups();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
};

/**
 * Create a backup
 */
async function handleCreateBackup() {
    const desc = await Modal.show({
        title: 'Create Backup',
        message: 'Enter a description for this backup:',
        input: true,
        inputValue: `Backup ${new Date().toLocaleString()}`
    });
    // desc is `false` when cancelled, or a string (possibly empty) when confirmed
    if (desc === false) return;
    const description = (typeof desc === 'string' && desc.trim()) ? desc.trim() : `Backup ${new Date().toLocaleString()}`;
    
    showStatus('Creating backup...');
    
    try {
        const result = await invoke('create_backup', { description: description });
        showStatus(result);
        await loadBackups();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
}

/**
 * Load bloatware data
 */
async function loadBloatware() {
    showLoading(DOM.lists.bloatware);
    
    try {
        const items = await invoke('get_bloatware');
        AppState.bloatwareData = Array.isArray(items) ? items : [];
        renderBloatware();
    } catch (err) {
        console.error('[loadBloatware] Error:', err);
        showError(DOM.lists.bloatware, 'Failed to load bloatware: ' + err);
    }
}

/**
 * Render bloatware list with filtering
 */
function renderBloatware() {
    if (!AppState.bloatwareData.length) {
        renderList(DOM.lists.bloatware, [], () => '');
        return;
    }
    
    const filtered = AppState.debloatCategory === 'all' 
        ? AppState.bloatwareData 
        : AppState.bloatwareData.filter(item => item.subcategory === AppState.debloatCategory);
    
    renderList(DOM.lists.bloatware, filtered, (item, i) => {
        const isFav = AppState.favorites.has(`bloatware-${item.id}`);
        
        return renderCard({
            title: item.name,
            description: escapeHtml(item.description),
            badge: 'Debloat',
            statusText: 'Installed',
            statusActive: false,
            hasButton: true,
            buttonText: 'Remove',
            buttonData: `data-id="${escapeHtml(item.id)}" data-name="${escapeHtml(item.name)}"`,
            buttonClass: 'danger remove-bloat-btn',
            favType: 'bloatware',
            favIndex: item.id,
            isFav
        });
    });
}

/**
 * Remove bloatware
 */
window.removeBloatware = async function(wingetId, name) {
    showStatus(`Removing ${name}...`);
    showProgress(30);
    
    try {
        const result = await invoke('remove_bloatware', { name: wingetId });
        showProgress(100);
        setTimeout(() => hideProgress(), 500);
        showStatus(result);
        await loadBloatware();
    } catch (err) {
        hideProgress();
        showStatus(`Error: ${err}`);
    }
};

/**
 * Load software data
 */
async function loadSoftware() {
    showLoading(DOM.lists.software);
    
    try {
        const items = await invoke('get_software');
        AppState.softwareData = Array.isArray(items) ? items : [];
        renderSoftware();
    } catch (err) {
        console.error('[loadSoftware] Error:', err);
        showError(DOM.lists.software, 'Failed to load software: ' + err);
    }
}

/**
 * Render software list with filtering
 */
function renderSoftware() {
    if (!AppState.softwareData.length) {
        renderList(DOM.lists.software, [], () => '');
        return;
    }
    
    const filtered = AppState.installerCategory === 'all' 
        ? AppState.softwareData 
        : AppState.softwareData.filter(item => item.category === AppState.installerCategory);
    
    renderList(DOM.lists.software, filtered, (item, i) => {
        const isFav = AppState.favorites.has(`software-${item.id}`);
        
        return renderCard({
            title: item.name,
            description: escapeHtml(item.winget_id),
            badge: escapeHtml(item.category),
            statusText: 'Available',
            statusActive: false,
            hasButton: true,
            buttonText: 'Install',
            buttonData: `data-name="${escapeHtml(item.name)}" data-winget="${escapeHtml(item.winget_id)}"`,
            buttonClass: 'install-btn',
            favType: 'software',
            favIndex: item.id,
            isFav
        });
    });
}

/**
 * Install software
 */
window.installSoftware = async function(name, wingetId) {
    showStatus(`Installing ${name}...`);
    
    try {
        const result = await invoke('install_software', { winget_id: wingetId });
        showStatus(result);
        await loadSoftware();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
};

/**
 * Load audit log
 */
async function loadAuditLog() {
    const auditList = document.getElementById('audit-list');
    showLoading(auditList);
    
    try {
        const entries = await invoke('get_audit_log', { limit: 200 });
        
        if (!entries || entries.length === 0) {
            auditList.innerHTML = '<p class="empty-state">No audit entries yet</p>';
            return;
        }
        
        auditList.innerHTML = entries.map(entry => {
            const levelClass = entry.level === 'error' ? 'error' : entry.level === 'warn' ? 'warning' : 'info';
            const time = new Date(entry.timestamp).toLocaleString();
            
            return `
                <div class="audit-entry ${levelClass}" role="listitem">
                    <div class="audit-entry-header">
                        <span class="audit-level ${levelClass}">${entry.level.toUpperCase()}</span>
                        <span class="audit-category">${escapeHtml(entry.category)}</span>
                        <span class="audit-time">${time}</span>
                    </div>
                    <div class="audit-message">${escapeHtml(entry.message)}</div>
                    ${entry.details ? `<div class="audit-details">${escapeHtml(entry.details)}</div>` : ''}
                </div>
            `;
        }).join('');
    } catch (err) {
        showError(auditList, 'Failed to load audit log');
    }
}

/**
 * Clear audit log
 */
async function handleClearAudit() {
    const confirmed = await Modal.show({ title: 'Clear Audit Log', message: 'Clear all audit log entries? This cannot be undone.' });
    if (!confirmed) return;
    
    showStatus('Clearing audit log...');
    
    try {
        const result = await invoke('clear_audit_log');
        showStatus(result);
        await loadAuditLog();
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
}

/**
 * Handle uninstall
 */
async function handleUninstall() {
    const confirmed1 = await Modal.show({
        title: 'Uninstall ZingerBoost',
        message: 'WARNING: This will delete all ZingerBoost data, settings, and backups.\n\nThis action cannot be undone. Are you sure?',
        danger: true
    });
    if (!confirmed1) return;

    const confirmed2 = await Modal.show({
        title: 'Final Confirmation',
        message: 'Delete ALL ZingerBoost data permanently?',
        danger: true
    });
    if (!confirmed2) return;
    
    showStatus('Uninstalling ZingerBoost...');
    
    try {
        const result = await invoke('uninstall_app');
        showStatus(result + ' — Please close this window');
    } catch (err) {
        showStatus(`Error: ${err}`);
    }
}

/**
 * Check for updates
 */
async function handleCheckUpdate() {
    const statusEl = document.getElementById('update-status');
    if (statusEl) statusEl.innerHTML = '<span style="color: var(--text-tertiary)">Checking...</span>';
    
    try {
        const result = await invoke('check_for_updates');
        
        if (statusEl) {
            if (result.has_update) {
                statusEl.innerHTML = `<span style="color: var(--text-primary)">Update available: v${result.latest}</span>`;
            } else {
                statusEl.innerHTML = '<span style="color: var(--text-secondary)">You have the latest version</span>';
            }
        }
        
        showStatus(result.has_update ? `Update available: v${result.latest}` : 'You have the latest version');
    } catch (err) {
        if (statusEl) {
            statusEl.innerHTML = '<span style="color: var(--text-tertiary)">Could not check for updates</span>';
        }
        showStatus(`Error checking for updates: ${err}`);
    }
}

/**
 * Load favorites from backend persistence
 */
async function loadFavorites() {
    try {
        const favs = await invoke('get_favorites');
        AppState.favorites = new Set(Array.isArray(favs) ? favs : []);
        // Re-render current lists to update star states
        renderTweaks();
        renderServices();
    } catch (err) {
        console.error('[favorites] Failed to load:', err);
    }
}

/**
 * Toggle favorite status with backend persistence
 */
async function toggleFavorite(type, index) {
    const key = `${type}-${index}`;
    
    try {
        const result = await invoke('toggle_favorite', { key });
        const isNowFav = result.is_favorite;
        
        if (isNowFav) {
            AppState.favorites.add(key);
        } else {
            AppState.favorites.delete(key);
        }
        
        // Update all star buttons for this item
        document.querySelectorAll(`.fav-btn[data-type="${type}"][data-index="${index}"]`).forEach(btn => {
            btn.classList.toggle('active', isNowFav);
            btn.setAttribute('aria-label', isNowFav ? 'Remove from favorites' : 'Add to favorites');
            const svg = btn.querySelector('svg');
            if (svg) svg.setAttribute('fill', isNowFav ? 'currentColor' : 'none');
        });
        
        // If on favorites tab, re-render
        if (AppState.currentTab === 'favorites') {
            renderFavorites();
        }
    } catch (err) {
        console.error('[favorites] Toggle failed:', err);
    }
}

/**
 * Render favorites tab - shows all favorited items with action buttons
 */
function renderFavorites() {
    const container = DOM.lists.favorites;
    if (!container) return;
    
    const favKeys = Array.from(AppState.favorites);
    
    if (favKeys.length === 0) {
        container.innerHTML = '<p class="empty-state">No favorites yet. Click the star icon on any item to add it here.</p>';
        return;
    }
    
    // Group favorites by type
    const grouped = {};
    for (const key of favKeys) {
        // Key format: "type-itemId" where itemId may contain hyphens
        const dashIndex = key.indexOf('-');
        if (dashIndex === -1) continue;
        const type = key.substring(0, dashIndex);
        const itemId = key.substring(dashIndex + 1);
        if (!grouped[type]) grouped[type] = [];
        grouped[type].push({ key, itemId });
    }
    
    const typeLabels = {
        tweak: 'Optimizations',
        service: 'Services',
        cleaner: 'Cleaner',
        bloatware: 'Debloat',
        software: 'Installer'
    };
    
    const typeSources = {
        tweak: AppState.tweaksData,
        service: AppState.servicesData,
        cleaner: AppState.cleanerData,
        bloatware: AppState.bloatwareData,
        software: AppState.softwareData
    };
    
    let html = '';
    
    for (const [type, items] of Object.entries(grouped)) {
        const label = typeLabels[type] || type;
        const source = typeSources[type] || [];
        
        html += `<div class="fav-group"><h3>${escapeHtml(label)}</h3><div class="fav-grid">`;
        
        for (const item of items) {
            // Look up data by item ID instead of array index
            const idKey = type === 'service' ? 'name' : 'id';
            const data = source.find(s => s[idKey] === item.itemId);
            if (!data) continue;
            
            if (type === 'tweak') {
                html += renderFavCard({
                    title: data.name,
                    description: data.description,
                    badge: catLabels[data.category] || data.category,
                    statusText: 'See Optimizations',
                    hasButton: true,
                    buttonText: 'Go to',
                    buttonClass: 'btn-action',
                    buttonData: `onclick="switchTab('optimizations')"`,
                    favType: type,
                    favIndex: item.itemId
                });
            } else if (type === 'service') {
                const isRunning = data.status === 'Running';
                html += renderFavCard({
                    title: data.display_name,
                    description: isRunning ? 'Currently running' : 'Currently stopped',
                    badge: 'Service',
                    statusText: data.status,
                    statusActive: isRunning,
                    hasToggle: true,
                    toggleChecked: isRunning,
                    toggleClass: 'service-toggle',
                    toggleData: `data-index="${item.itemId}" data-name="${escapeHtml(data.name)}" data-display="${escapeHtml(data.display_name)}"`,
                    favType: type,
                    favIndex: item.itemId
                });
            } else if (type === 'cleaner') {
                html += renderFavCard({
                    title: data.name,
                    description: `${data.size_mb.toFixed(1)} MB reclaimable`,
                    badge: 'Cleaner',
                    statusText: data.risk,
                    statusActive: data.risk === 'safe',
                    hasButton: true,
                    buttonText: 'Clean',
                    buttonData: `data-id="${escapeHtml(data.id)}" data-name="${escapeHtml(data.name)}"`,
                    buttonClass: 'clean-btn',
                    favType: type,
                    favIndex: item.itemId
                });
            } else if (type === 'bloatware') {
                html += renderFavCard({
                    title: data.name,
                    description: escapeHtml(data.description),
                    badge: 'Debloat',
                    statusText: 'Installed',
                    hasButton: true,
                    buttonText: 'Remove',
                    buttonData: `data-id="${escapeHtml(data.id)}" data-name="${escapeHtml(data.name)}" data-winget="${escapeHtml(data.winget_id)}"`,
                    buttonClass: 'danger remove-bloat-btn',
                    favType: type,
                    favIndex: item.itemId
                });
            } else if (type === 'software') {
                html += renderFavCard({
                    title: data.name,
                    description: escapeHtml(data.winget_id),
                    badge: escapeHtml(data.category),
                    statusText: 'Available',
                    hasButton: true,
                    buttonText: 'Install',
                    buttonData: `data-name="${escapeHtml(data.name)}" data-winget="${escapeHtml(data.winget_id)}"`,
                    buttonClass: 'install-btn',
                    favType: type,
                    favIndex: item.itemId
                });
            }
        }
        
        html += '</div></div>';
    }
    
    container.innerHTML = html;
}

/**
 * Render a compact card for favorites
 */
function renderFavCard({
    title,
    description,
    badge,
    statusText,
    statusActive,
    hasToggle,
    toggleChecked,
    toggleClass,
    toggleData,
    hasButton,
    buttonText,
    buttonClass,
    buttonData,
    favType,
    favIndex
}) {
    const catClass = getCatClass(badge);
    const isFav = true;
    
    return `
        <div class="card fav-card" role="listitem">
            <div class="card-header">
                <div class="card-info">
                    <div class="card-badge ${catClass}">${escapeHtml(badge)}</div>
                    <h3>${escapeHtml(title)}</h3>
                    <p>${escapeHtml(description)}</p>
                </div>
                <button class="fav-btn active" 
                        data-type="${escapeHtml(favType)}" 
                        data-index="${favIndex}" 
                        title="Remove from favorites"
                        aria-label="Remove from favorites">
                    <svg viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
                    </svg>
                </button>
            </div>
            <div class="card-footer">
                <div class="card-status ${statusActive ? 'active' : 'inactive'}">
                    <span class="status-dot"></span>
                    <span>${escapeHtml(statusText)}</span>
                </div>
                ${hasToggle ? `
                    <label class="toggle-switch">
                        <input type="checkbox" 
                               class="${toggleClass}" 
                               ${toggleData} 
                               ${toggleChecked ? 'checked' : ''}
                               role="switch"
                               aria-checked="${toggleChecked}">
                        <span class="toggle-slider"></span>
                    </label>
                ` : ''}
                ${hasButton ? `
                    <button class="btn-action ${buttonClass || ''}" 
                            ${buttonData}>
                        ${escapeHtml(buttonText)}
                    </button>
                ` : ''}
            </div>
        </div>
    `;
}

/**
 * Update maximize/restore icon based on window state
 */
function updateMaximizeIcon(isMaximized) {
    AppState.isMaximized = isMaximized;

    if (DOM.iconMaximize && DOM.iconRestore) {
        if (isMaximized) {
            DOM.iconMaximize.classList.add('hidden');
            DOM.iconRestore.classList.remove('hidden');
            DOM.titlebarMaximize?.setAttribute('aria-label', 'Restore');
            DOM.titlebarMaximize?.setAttribute('title', 'Restore');
        } else {
            DOM.iconMaximize.classList.remove('hidden');
            DOM.iconRestore.classList.add('hidden');
            DOM.titlebarMaximize?.setAttribute('aria-label', 'Maximize');
            DOM.titlebarMaximize?.setAttribute('title', 'Maximize');
        }
    }
}

/**
 * Escape HTML to prevent XSS
 * Also escapes quotes for safe use in HTML attribute values
 */
function escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}
