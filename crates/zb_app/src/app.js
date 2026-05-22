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
    isMaximized: false,
    // Debloat status tracking (Map<bloatwareId, 'installed'|'removed'|'checking'>)
    bloatwareStatuses: new Map(),
    // Debloat multi-select
    debloatSelection: new Set(),
    // Whether bloatware statuses are currently being checked
    bloatwareChecking: false
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
    setupResizeObserver();
    loadAppInfo();
    checkAdminStatus();
    loadInitialData();
    await startMetricsPolling();
});

/**
 * Setup ResizeObserver for dynamic layout adjustments
 * Adjusts card grid min-width based on available content area
 */
function setupResizeObserver() {
    const content = document.getElementById('content');
    if (!content || typeof ResizeObserver === 'undefined') return;

    const observer = new ResizeObserver((entries) => {
        for (const entry of entries) {
            const width = entry.contentRect.width;
            const root = document.documentElement;

            // Dynamic card min-width based on content area width
            if (width >= 1200) {
                root.style.setProperty('--card-min-width', '280px');
            } else if (width >= 900) {
                root.style.setProperty('--card-min-width', '250px');
            } else if (width >= 600) {
                root.style.setProperty('--card-min-width', '210px');
            } else if (width >= 400) {
                root.style.setProperty('--card-min-width', '170px');
            } else if (width >= 280) {
                root.style.setProperty('--card-min-width', '140px');
            } else {
                root.style.setProperty('--card-min-width', '120px');
            }

            // Add a class to body for JS-based responsive logic
            document.body.classList.toggle('compact-layout', width < 500);
            document.body.classList.toggle('wide-layout', width >= 1200);
        }
    });

    observer.observe(content);
}

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
                // Clear selection when switching categories
                AppState.debloatSelection.clear();
                renderBloatware();
            });
        });
    }
    // Update debloat tab badges with counts
    updateDebloatTabBadges();

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
        } else if (btn.id === 'debloat-remove-selected') {
            window.removeSelectedBloatware();
        } else if (btn.id === 'debloat-remove-all') {
            window.removeAllBloatware();
        } else if (btn.id === 'debloat-refresh-status') {
            // Re-check all statuses
            for (const item of AppState.bloatwareData) {
                AppState.bloatwareStatuses.set(item.id, 'checking');
                updateBloatwareCardStatus(item.id);
            }
            checkAllBloatwareStatuses();
        }
    });

    // Debloat toolbar "Select All" button delegation (styled button, not checkbox)
    document.addEventListener('click', (e) => {
        const selectAllBtn = e.target.closest('#debloat-select-all-btn');
        if (selectAllBtn) {
            const filtered = getFilteredBloatware();
            const selectable = filtered.filter(item => {
                const status = AppState.bloatwareStatuses.get(item.id);
                return status !== 'removed';
            });
            const allSelected = selectable.length > 0 && selectable.every(item => AppState.debloatSelection.has(item.id));
            
            AppState.debloatSelection.clear();
            if (!allSelected) {
                for (const item of selectable) {
                    AppState.debloatSelection.add(item.id);
                }
            }
            renderBloatware();
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
        if (info.version) {
            const versionStr = `v${info.version}`;
            // Update settings tab version
            const settingsVersionEl = document.getElementById('app-version');
            if (settingsVersionEl) settingsVersionEl.textContent = versionStr;
            // Update sidebar version
            const sidebarVersionEl = document.getElementById('version-display');
            if (sidebarVersionEl) sidebarVersionEl.textContent = versionStr;
        }
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
        debloat: () => {
            // Only reload if data hasn't been loaded yet
            if (AppState.bloatwareData.length === 0) {
                return loadBloatware();
            }
            // Otherwise just re-render (status checks may still be running)
            renderBloatware();
        },
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
    if (!metrics || typeof metrics !== 'object') return;
    
    // Safety: ensure all values are valid numbers (guard against NaN/undefined/null)
    const cpuPct = (typeof metrics.cpu_percent === 'number' && isFinite(metrics.cpu_percent)) ? metrics.cpu_percent : 0;
    const ramPct = (typeof metrics.ram_percent === 'number' && isFinite(metrics.ram_percent)) ? metrics.ram_percent : 0;
    const ramUsed = (typeof metrics.ram_used_mb === 'number' && isFinite(metrics.ram_used_mb)) ? metrics.ram_used_mb : 0;
    const ramTotal = (typeof metrics.ram_total_mb === 'number' && isFinite(metrics.ram_total_mb)) ? metrics.ram_total_mb : 0;
    const diskPct = (typeof metrics.disk_active_percent === 'number' && isFinite(metrics.disk_active_percent)) ? metrics.disk_active_percent : 0;
    const netDown = (typeof metrics.network_down_mbps === 'number' && isFinite(metrics.network_down_mbps)) ? metrics.network_down_mbps : 0;
    const netUp = (typeof metrics.network_up_mbps === 'number' && isFinite(metrics.network_up_mbps)) ? metrics.network_up_mbps : 0;
    const netMbps = netDown + netUp;
    
    // CPU
    const cpuVal = document.getElementById('home-cpu-value');
    const cpuBar = document.getElementById('home-cpu-bar');
    if (cpuVal) { cpuVal.textContent = `${Math.round(cpuPct)}%`; flashElement(cpuVal); }
    if (cpuBar) { cpuBar.style.width = `${Math.min(Math.max(cpuPct, 0), 100)}%`; cpuBar.parentElement.setAttribute('aria-valuenow', Math.round(cpuPct)); }
    
    // RAM — show "X.X / Y.Y GB" format + percentage
    const ramVal = document.getElementById('home-ram-value');
    const ramBar = document.getElementById('home-ram-bar');
    if (ramVal) {
        const usedGB = (ramUsed / 1024).toFixed(1);
        const totalGB = (ramTotal / 1024).toFixed(1);
        ramVal.textContent = ramTotal > 0 ? `${usedGB} / ${totalGB} GB` : `${Math.round(ramPct)}%`;
        flashElement(ramVal);
    }
    if (ramBar) { ramBar.style.width = `${Math.min(Math.max(ramPct, 0), 100)}%`; ramBar.parentElement.setAttribute('aria-valuenow', Math.round(ramPct)); }
    
    // Disk
    const diskVal = document.getElementById('home-disk-value');
    const diskBar = document.getElementById('home-disk-bar');
    if (diskVal) { diskVal.textContent = `${Math.round(diskPct)}%`; flashElement(diskVal); }
    if (diskBar) { diskBar.style.width = `${Math.min(Math.max(diskPct, 0), 100)}%`; diskBar.parentElement.setAttribute('aria-valuenow', Math.round(diskPct)); }
    
    // Network
    const netVal = document.getElementById('home-network-value');
    const netBar = document.getElementById('home-network-bar');
    if (netVal) {
        if (netMbps < 0.001) {
            netVal.textContent = '0 Kbps';
        } else if (netMbps < 1) {
            netVal.textContent = `${(netMbps * 1000).toFixed(0)} Kbps`;
        } else {
            netVal.textContent = `${netMbps.toFixed(1)} Mbps`;
        }
        flashElement(netVal);
    }
    if (netBar) { const np = Math.min(Math.max(netMbps * 5, 0), 100); netBar.style.width = `${np}%`; netBar.parentElement.setAttribute('aria-valuenow', Math.round(np)); }
}

function flashElement(el) {
    if (!el) return;
    el.classList.add('metric-updated');
    clearTimeout(el._flashTimeout);
    el._flashTimeout = setTimeout(() => el.classList.remove('metric-updated'), 300);
}

/**
 * Start metrics updates. Backend emits 'metrics-update' event every second.
 * This function sets up the event listener and does initial load.
 */
async function startMetricsPolling() {
    // Initial load
    await loadMetrics();
    
    // Listen for metrics updates from backend via Tauri events
    if (window.__TAURI__ && window.__TAURI__.event) {
        try {
            await window.__TAURI__.event.listen('metrics-update', (event) => {
                let metrics = event.payload;
                // Payload may be a JSON string (if backend emits via current_json())
                // or an object (if backend emits the struct directly). Handle both.
                if (typeof metrics === 'string') {
                    try {
                        metrics = JSON.parse(metrics);
                    } catch (e) {
                        console.warn('[metrics] Failed to parse event payload:', e);
                        return;
                    }
                }
                if (metrics && typeof metrics === 'object') {
                    updateHomeMetrics(metrics);
                    
                    const tsEl = document.getElementById('metrics-last-update');
                    if (tsEl) {
                        const now = new Date();
                        tsEl.textContent = now.toLocaleTimeString();
                    }
                }
            });
            console.log('[metrics] Event listener registered for metrics-update');
        } catch (err) {
            console.error('[metrics] Failed to register event listener:', err);
        }
    }
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
    
    renderList(DOM.lists.services, filtered, (svc) => {
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
            toggleData: `data-index="${escapeHtml(svc.name)}" data-name="${escapeHtml(svc.name)}" data-display="${escapeHtml(svc.display_name)}"`,
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
    // Find ALL matching toggles (may exist in both Services and Favorites tabs)
    const toggles = document.querySelectorAll(`.service-toggle[data-index="${index}"]`);
    if (toggles.length === 0) return;
    
    const primaryToggle = toggles[0];
    const shouldBeActive = primaryToggle.checked;
    showStatus(shouldBeActive ? `Starting ${displayName}...` : `Stopping ${displayName}...`);
    
    // Disable all matching toggles
    toggles.forEach(t => t.disabled = true);
    
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
        
        // Also refresh favorites if on that tab
        if (AppState.currentTab === 'favorites') {
            renderFavorites();
        }
    } catch (err) {
        showStatus(`Error: ${err}`);
        // Revert all matching toggles
        toggles.forEach(t => t.checked = !shouldBeActive);
    } finally {
        toggles.forEach(t => t.disabled = false);
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
            buttonData: `data-id="${escapeHtml(item.id)}" data-name="${escapeHtml(item.name)}"`,
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
    if (DOM.lists.bloatware) {
        DOM.lists.bloatware.innerHTML = `
            <div class="loading-spinner">
                <div class="spinner"></div>
                <p>Loading bloatware list...</p>
            </div>
        `;
    }
    
    try {
        const items = await invoke('get_bloatware');
        AppState.bloatwareData = Array.isArray(items) ? items : [];
        
        // Initialize statuses as 'checking' for all items
        for (const item of AppState.bloatwareData) {
            if (!AppState.bloatwareStatuses.has(item.id)) {
                AppState.bloatwareStatuses.set(item.id, 'checking');
            }
        }
        
        renderBloatware();
        
        // Check installed status for all items in background
        checkAllBloatwareStatuses();
    } catch (err) {
        console.error('[loadBloatware] Error:', err);
        if (DOM.lists.bloatware) {
            DOM.lists.bloatware.innerHTML = `<p class="empty-state error">Failed to load bloatware: ${escapeHtml(err)}</p>`;
        }
    }
}

/**
 * Check installed status for all bloatware items in parallel batches
 */
async function checkAllBloatwareStatuses() {
    if (AppState.bloatwareChecking) return;
    AppState.bloatwareChecking = true;
    
    const items = AppState.bloatwareData;
    const BATCH_SIZE = 5; // Check 5 at a time to avoid overwhelming the system
    
    for (let i = 0; i < items.length; i += BATCH_SIZE) {
        const batch = items.slice(i, i + BATCH_SIZE);
        const promises = batch.map(async (item) => {
            try {
                // Items with empty winget_id (like "Remove Ads & Widgets") use special handling
                const installed = await invoke('check_bloatware_installed', { wingetId: item.winget_id });
                AppState.bloatwareStatuses.set(item.id, installed ? 'installed' : 'removed');
            } catch (err) {
                console.warn(`[checkBloatwareStatus] Failed for ${item.name}:`, err);
                AppState.bloatwareStatuses.set(item.id, 'unknown');
            }
            // Update the card status in DOM without full re-render
            updateBloatwareCardStatus(item.id);
        });
        
        await Promise.allSettled(promises);
    }
    
    AppState.bloatwareChecking = false;
    updateDebloatToolbar();
}

/**
 * Update a single bloatware card's status indicator in the DOM
 */
function updateBloatwareCardStatus(itemId) {
    const card = document.querySelector(`.card[data-bloat-id="${itemId}"]`);
    if (!card) return;
    
    const status = AppState.bloatwareStatuses.get(itemId) || 'unknown';
    const statusEl = card.querySelector('.card-status');
    const removeBtn = card.querySelector('.remove-bloat-btn');
    
    if (statusEl) {
        const isInstalled = status === 'installed';
        const isRemoved = status === 'removed';
        const isChecking = status === 'checking';
        
        statusEl.className = `card-status ${isInstalled ? 'active' : 'inactive'}`;
        statusEl.innerHTML = `
            <span class="status-dot"></span>
            <span>${isChecking ? 'Checking...' : isRemoved ? 'Removed' : isInstalled ? 'Installed' : 'Unknown'}</span>
        `;
    }
    
    // Disable remove button if already removed
    if (removeBtn) {
        if (status === 'removed') {
            removeBtn.disabled = true;
            removeBtn.textContent = 'Removed';
        } else if (status === 'checking') {
            removeBtn.disabled = true;
        } else {
            removeBtn.disabled = false;
            removeBtn.textContent = 'Remove';
        }
    }
}

/**
 * Render bloatware list with filtering
 */
function renderBloatware() {
    if (!AppState.bloatwareData.length) {
        DOM.lists.bloatware.innerHTML = '<p class="empty-state">No bloatware items available</p>';
        return;
    }
    
    // Filter by category or status
    let filtered;
    if (AppState.debloatCategory === 'all') {
        filtered = AppState.bloatwareData;
    } else if (AppState.debloatCategory === 'installed') {
        filtered = AppState.bloatwareData.filter(item => AppState.bloatwareStatuses.get(item.id) === 'installed');
    } else if (AppState.debloatCategory === 'removed') {
        filtered = AppState.bloatwareData.filter(item => AppState.bloatwareStatuses.get(item.id) === 'removed');
    } else {
        filtered = AppState.bloatwareData.filter(item => item.subcategory === AppState.debloatCategory);
    }
    
    // Render toolbar
    const toolbarHtml = renderDebloatToolbar(filtered);
    
    // Render cards
    const cardsHtml = filtered.map((item) => {
        const isFav = AppState.favorites.has(`bloatware-${item.id}`);
        const status = AppState.bloatwareStatuses.get(item.id) || 'checking';
        const isInstalled = status === 'installed';
        const isRemoved = status === 'removed';
        const isChecking = status === 'checking';
        const isSelected = AppState.debloatSelection.has(item.id);
        
        const statusText = isChecking ? 'Checking...' : isRemoved ? 'Removed' : isInstalled ? 'Installed' : 'Unknown';
        const btnDisabled = isRemoved || isChecking;
        const btnText = isRemoved ? 'Removed' : 'Remove';
        
        return `
            <div class="card ${isSelected ? 'card-selected' : ''} has-checkbox" 
                 role="listitem" data-bloat-id="${escapeHtml(item.id)}">
                <label class="card-checkbox" onclick="event.stopPropagation()">
                    <input type="checkbox" 
                           ${isSelected ? 'checked' : ''}
                           ${btnDisabled ? 'disabled' : ''}
                           data-bloat-check-id="${escapeHtml(item.id)}"
                           aria-label="Select ${escapeHtml(item.name)}">
                </label>
                <div class="card-header">
                    <div class="card-info">
                        <div class="card-badge cat-debloat">Debloat</div>
                        <h3>${escapeHtml(item.name)}</h3>
                        <p>${escapeHtml(item.description)}</p>
                    </div>
                    <button class="fav-btn ${isFav ? 'active' : ''}" 
                            data-type="bloatware" 
                            data-index="${escapeHtml(item.id)}" 
                            title="${isFav ? 'Remove from favorites' : 'Add to favorites'}"
                            aria-label="${isFav ? 'Remove from favorites' : 'Add to favorites'}">
                        <svg viewBox="0 0 24 24" fill="${isFav ? 'currentColor' : 'none'}" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
                        </svg>
                    </button>
                </div>
                <div class="card-footer">
                    <div class="card-status ${isInstalled ? 'active' : 'inactive'}">
                        <span class="status-dot"></span>
                        <span>${statusText}</span>
                    </div>
                    <button class="btn-action danger remove-bloat-btn" 
                            data-id="${escapeHtml(item.id)}" 
                            data-name="${escapeHtml(item.name)}"
                            data-winget="${escapeHtml(item.winget_id)}"
                            ${btnDisabled ? 'disabled' : ''}>
                        ${btnText}
                    </button>
                </div>
            </div>
        `;
    }).join('');
    
    DOM.lists.bloatware.innerHTML = toolbarHtml + `<div class="cards-grid">${cardsHtml}</div>`;
    
    // Bind checkbox events
    DOM.lists.bloatware.querySelectorAll('input[data-bloat-check-id]').forEach(cb => {
        cb.addEventListener('change', (e) => {
            const id = e.target.dataset.bloatCheckId;
            if (e.target.checked) {
                AppState.debloatSelection.add(id);
            } else {
                AppState.debloatSelection.delete(id);
            }
            const card = e.target.closest('.card');
            if (card) card.classList.toggle('card-selected', e.target.checked);
            updateDebloatToolbar();
        });
    });
    
    updateDebloatToolbar();
}

/**
 * Render the debloat toolbar with styled Select All button, remove selected, remove all
 */
function renderDebloatToolbar(filtered) {
    const totalItems = filtered || getFilteredBloatware();
    const selectable = totalItems.filter(item => {
        const status = AppState.bloatwareStatuses.get(item.id);
        return status !== 'removed';
    });
    // Count only selected items within the current filter
    const selectedCount = totalItems.filter(item => AppState.debloatSelection.has(item.id)).length;
    const allSelected = selectable.length > 0 && selectable.every(item => AppState.debloatSelection.has(item.id));
    
    // Check/uncheck SVG icons
    const checkIcon = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>`;
    const uncheckIcon = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/></svg>`;
    
    return `
        <div class="debloat-toolbar">
            <button class="toolbar-select-btn ${allSelected ? 'active' : ''}" id="debloat-select-all-btn" title="Select all items">
                ${allSelected ? checkIcon : uncheckIcon}
                <span>Select All</span>
            </button>
            <span class="toolbar-count">${selectedCount} selected</span>
            <span class="toolbar-spacer"></span>
            <button class="btn-secondary" id="debloat-refresh-status" title="Recheck installed status">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10"/><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/></svg>
                Refresh
            </button>
            <button class="btn-danger" id="debloat-remove-selected" ${selectedCount === 0 ? 'disabled' : ''}>
                Remove Selected (${selectedCount})
            </button>
            <button class="btn-primary" id="debloat-remove-all">
                Remove All
            </button>
        </div>
    `;
}

/**
 * Get the currently filtered bloatware list based on active category
 */
function getFilteredBloatware() {
    if (AppState.debloatCategory === 'all') {
        return AppState.bloatwareData;
    } else if (AppState.debloatCategory === 'installed') {
        return AppState.bloatwareData.filter(item => AppState.bloatwareStatuses.get(item.id) === 'installed');
    } else if (AppState.debloatCategory === 'removed') {
        return AppState.bloatwareData.filter(item => AppState.bloatwareStatuses.get(item.id) === 'removed');
    } else {
        return AppState.bloatwareData.filter(item => item.subcategory === AppState.debloatCategory);
    }
}

/**
 * Update debloat tab badges with item counts
 */
function updateDebloatTabBadges() {
    const debloatTabs = document.getElementById('debloat-tabs');
    if (!debloatTabs) return;
    
    const all = AppState.bloatwareData.length;
    const games = AppState.bloatwareData.filter(i => i.subcategory === 'games').length;
    const apps = AppState.bloatwareData.filter(i => i.subcategory === 'apps').length;
    const system = AppState.bloatwareData.filter(i => i.subcategory === 'system').length;
    const installed = AppState.bloatwareData.filter(i => AppState.bloatwareStatuses.get(i.id) === 'installed').length;
    const removed = AppState.bloatwareData.filter(i => AppState.bloatwareStatuses.get(i.id) === 'removed').length;
    
    const counts = { all, games, apps, system, installed, removed };
    
    debloatTabs.querySelectorAll('.opt-tab').forEach(btn => {
        const cat = btn.dataset.cat;
        const count = counts[cat] || 0;
        // Remove existing badge
        const existing = btn.querySelector('.tab-badge');
        if (existing) existing.remove();
        // Add new badge
        if (count > 0) {
            const badge = document.createElement('span');
            badge.className = 'tab-badge';
            badge.textContent = count;
            btn.appendChild(badge);
        }
    });
}

/**
 * Update the debloat toolbar without re-rendering the entire list
 */
function updateDebloatToolbar() {
    const filtered = getFilteredBloatware();
    
    const selectable = filtered.filter(item => {
        const status = AppState.bloatwareStatuses.get(item.id);
        return status !== 'removed';
    });
    // Count only selected items within the current filter
    const selectedCount = filtered.filter(item => AppState.debloatSelection.has(item.id)).length;
    const allSelected = selectable.length > 0 && selectable.every(item => AppState.debloatSelection.has(item.id));
    
    // Update styled Select All button
    const selectAllBtn = document.getElementById('debloat-select-all-btn');
    if (selectAllBtn) {
        selectAllBtn.classList.toggle('active', allSelected);
        const checkIcon = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>`;
        const uncheckIcon = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/></svg>`;
        selectAllBtn.innerHTML = `${allSelected ? checkIcon : uncheckIcon}<span>Select All</span>`;
    }
    
    const countEl = document.querySelector('.debloat-toolbar .toolbar-count');
    if (countEl) countEl.textContent = `${selectedCount} selected`;
    
    const removeSelectedBtn = document.getElementById('debloat-remove-selected');
    if (removeSelectedBtn) {
        removeSelectedBtn.disabled = selectedCount === 0;
        removeSelectedBtn.textContent = `Remove Selected (${selectedCount})`;
    }
    
    // Update tab badges
    updateDebloatTabBadges();
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
        
        // Update status for the removed item
        const item = AppState.bloatwareData.find(b => b.winget_id === wingetId || b.name === name);
        if (item) {
            AppState.bloatwareStatuses.set(item.id, 'removed');
            AppState.debloatSelection.delete(item.id);
            updateBloatwareCardStatus(item.id);
            updateDebloatToolbar();
        }
    } catch (err) {
        hideProgress();
        showStatus(`Error: ${err}`);
    }
};

/**
 * Remove selected bloatware items
 */
window.removeSelectedBloatware = async function() {
    const selectedIds = Array.from(AppState.debloatSelection);
    if (selectedIds.length === 0) return;
    
    const selectedItems = selectedIds
        .map(id => AppState.bloatwareData.find(b => b.id === id))
        .filter(Boolean);
    
    const confirmed = await Modal.show({
        title: 'Remove Selected',
        message: `Remove ${selectedItems.length} selected bloatware item(s)?\n\n${selectedItems.map(i => '• ' + i.name).join('\n')}`,
        danger: true
    });
    if (!confirmed) return;
    
    showStatus(`Removing ${selectedItems.length} items...`);
    showProgress(10);
    
    let removed = 0;
    let failed = 0;
    
    for (let i = 0; i < selectedItems.length; i++) {
        const item = selectedItems[i];
        const progress = 10 + Math.round((i / selectedItems.length) * 85);
        showProgress(progress);
        showStatus(`Removing ${item.name}... (${i + 1}/${selectedItems.length})`);
        
        try {
            await invoke('remove_bloatware', { name: item.winget_id });
            AppState.bloatwareStatuses.set(item.id, 'removed');
            removed++;
        } catch (err) {
            console.error(`[removeSelected] Failed for ${item.name}:`, err);
            failed++;
        }
        
        updateBloatwareCardStatus(item.id);
    }
    
    AppState.debloatSelection.clear();
    showProgress(100);
    setTimeout(() => hideProgress(), 500);
    showStatus(`Removed ${removed} item(s)${failed > 0 ? `, ${failed} failed` : ''}`);
    updateDebloatToolbar();
};

/**
 * Remove all bloatware items in current view
 */
window.removeAllBloatware = async function() {
    const filtered = getFilteredBloatware();
    
    const removable = filtered.filter(item => {
        const status = AppState.bloatwareStatuses.get(item.id);
        return status !== 'removed';
    });
    
    if (removable.length === 0) {
        showStatus('No items to remove');
        return;
    }
    
    const confirmed = await Modal.show({
        title: 'Remove All',
        message: `Remove ALL ${removable.length} bloatware item(s) in "${AppState.debloatCategory === 'all' ? 'All' : AppState.debloatCategory}" category?\n\nThis action cannot be undone.`,
        danger: true
    });
    if (!confirmed) return;
    
    showStatus(`Removing ${removable.length} items...`);
    showProgress(10);
    
    let removed = 0;
    let failed = 0;
    
    for (let i = 0; i < removable.length; i++) {
        const item = removable[i];
        const progress = 10 + Math.round((i / removable.length) * 85);
        showProgress(progress);
        showStatus(`Removing ${item.name}... (${i + 1}/${removable.length})`);
        
        try {
            await invoke('remove_bloatware', { name: item.winget_id });
            AppState.bloatwareStatuses.set(item.id, 'removed');
            removed++;
        } catch (err) {
            console.error(`[removeAll] Failed for ${item.name}:`, err);
            failed++;
        }
        
        updateBloatwareCardStatus(item.id);
    }
    
    AppState.debloatSelection.clear();
    showProgress(100);
    setTimeout(() => hideProgress(), 500);
    showStatus(`Removed ${removed} item(s)${failed > 0 ? `, ${failed} failed` : ''}`);
    updateDebloatToolbar();
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
                const bloatStatus = AppState.bloatwareStatuses.get(data.id);
                const bloatInstalled = bloatStatus === 'installed';
                const bloatRemoved = bloatStatus === 'removed';
                const bloatStatusText = bloatRemoved ? 'Removed' : bloatInstalled ? 'Installed' : (bloatStatus === 'checking' ? 'Checking...' : 'Unknown');
                html += renderFavCard({
                    title: data.name,
                    description: escapeHtml(data.description),
                    badge: 'Debloat',
                    statusText: bloatStatusText,
                    statusActive: bloatInstalled,
                    hasButton: true,
                    buttonText: bloatRemoved ? 'Removed' : 'Remove',
                    buttonData: `data-id="${escapeHtml(data.id)}" data-name="${escapeHtml(data.name)}" data-winget="${escapeHtml(data.winget_id)}"`,
                    buttonClass: 'danger remove-bloat-btn',
                    buttonDisabled: bloatRemoved,
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
    buttonDisabled,
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
