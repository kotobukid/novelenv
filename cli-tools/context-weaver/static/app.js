let files = [];
let currentContexts = [];
let narratives = [];
let currentTab = 'normal';

async function loadFiles() {
    try {
        const response = await fetch('/api/files');
        files = await response.json();
        renderFileTree();
    } catch (error) {
        console.error('Failed to load files:', error);
    }
}

async function loadNarratives() {
    try {
        const response = await fetch('/api/narratives');
        narratives = await response.json();
        renderNarratives();
    } catch (error) {
        console.error('Failed to load narratives:', error);
    }
}

function renderFileTree() {
    const treeView = document.getElementById('file-tree');
    treeView.innerHTML = '';

    const filesByPath = {};
    files.forEach(file => {
        filesByPath[file.path] = file;
    });

    const tree = buildTree(files);
    renderTreeNode(tree, treeView);
}

function buildTree(files) {
    const root = {name: '/', children: {}, files: []};

    files.forEach(file => {
        const parts = file.path.split('/');
        let current = root;

        if (file.is_directory) {
            parts.forEach(part => {
                if (!current.children[part]) {
                    current.children[part] = {name: part, children: {}, files: []};
                }
                current = current.children[part];
            });
        } else {
            const fileName = parts.pop();
            parts.forEach(part => {
                if (!current.children[part]) {
                    current.children[part] = {name: part, children: {}, files: []};
                }
                current = current.children[part];
            });
            current.files.push({name: fileName, path: file.path, preview: file.preview});
        }
    });

    return root;
}

function renderTreeNode(node, container, level = 0) {
    // Sort directories by name
    const sortedDirs = Object.values(node.children).sort((a, b) => a.name.localeCompare(b.name));
    sortedDirs.forEach(child => {
        const dirDiv = document.createElement('div');
        dirDiv.className = 'tree-item directory';
        dirDiv.style.paddingLeft = `${level * 20}px`;
        dirDiv.textContent = child.name;
        container.appendChild(dirDiv);

        renderTreeNode(child, container, level + 1);
    });

    // Sort files by name
    const sortedFiles = [...node.files].sort((a, b) => a.name.localeCompare(b.name));
    sortedFiles.forEach(file => {
        const fileDiv = document.createElement('div');
        fileDiv.className = 'tree-item file';
        fileDiv.style.paddingLeft = `${(level + 1) * 20}px`;
        fileDiv.innerHTML = `<div class="file-name">${file.name}</div><div class="file-preview">${file.preview}</div>`;
        fileDiv.draggable = true;
        fileDiv.dataset.path = file.path;

        fileDiv.addEventListener('dragstart', (e) => {
            e.dataTransfer.effectAllowed = 'copy';
            e.dataTransfer.setData('text/plain', file.path);
            fileDiv.classList.add('dragging');
        });

        fileDiv.addEventListener('dragend', () => {
            fileDiv.classList.remove('dragging');
        });

        container.appendChild(fileDiv);
    });
}

function setupContextList() {
    const contextList = document.getElementById('context-list');

    contextList.addEventListener('dragover', (e) => {
        // Check if this is a file being dragged from the tree (external drop)
        const hasFilePath = e.dataTransfer.types.includes('text/plain');
        const hasContextIndex = e.dataTransfer.types.includes('application/x-context-index');
        const isExternalFile = hasFilePath && !hasContextIndex;

        if (isExternalFile) {
            e.preventDefault();
            e.dataTransfer.dropEffect = 'copy';
            contextList.classList.add('drag-over');
        }
    });

    contextList.addEventListener('dragleave', (e) => {
        // Only remove drag-over if we're not entering a child element
        if (!contextList.contains(e.relatedTarget)) {
            contextList.classList.remove('drag-over');
        }
    });

    contextList.addEventListener('drop', (e) => {
        const path = e.dataTransfer.getData('text/plain');

        // Check if this is a new file being added (not a reorder operation)
        const isNewFile = path && !currentContexts.some(c => c.path === path);

        if (isNewFile) {
            e.preventDefault();
            e.stopPropagation();
            contextList.classList.remove('drag-over');

            currentContexts.push({
                path: path,
                include_type: {type: 'Full'},
                order: currentContexts.length
            });
            renderContextList();
            // console.log('Added new file:', path);
        }
    });
}

function renderContextList() {
    const contextList = document.getElementById('context-list');

    if (currentContexts.length === 0) {
        contextList.innerHTML = '<p class="placeholder">Drag files here to add to context</p>';
    } else {
        // console.log('Rendering context list. Current contexts:', currentContexts.map(c => `${c.path} (order: ${c.order})`));

        // Sort contexts by order (not by path anymore since we want user-defined order)
        const sortedContexts = [...currentContexts].sort((a, b) => a.order - b.order);
        // console.log('Sorted contexts:', sortedContexts.map(c => `${c.path} (order: ${c.order})`));

        let html = '';

        // Add drop zone at the beginning
        html += '<div class="drop-zone" data-position="0">ここに移動</div>';

        sortedContexts.forEach((context, displayIndex) => {
            const actualIndex = currentContexts.indexOf(context);
            html += `
                <div class="context-item" draggable="true" data-index="${actualIndex}">
                    <span class="drag-handle">⋮⋮</span>
                    <span class="path">${context.path}</span>
                    <span class="remove" onclick="removeContext(${actualIndex})">✕</span>
                </div>
            `;

            // Add drop zone after each item (except the last one will be handled by the list-end drop zone)
            html += `<div class="drop-zone" data-position="${displayIndex + 1}">ここに移動</div>`;
        });

        contextList.innerHTML = html;

        // Add drag and drop event listeners
        setupContextDragAndDrop();
    }
}

function setupContextDragAndDrop() {
    const contextItems = document.querySelectorAll('.context-item[draggable="true"]');
    const dropZones = document.querySelectorAll('.drop-zone');

    // Setup context item drag events
    contextItems.forEach((item) => {
        item.addEventListener('dragstart', (e) => {
            // console.log('Drag started for item:', item.dataset.index);
            e.dataTransfer.effectAllowed = 'move';
            e.dataTransfer.setData('application/x-context-index', item.dataset.index);
            item.classList.add('dragging');

            // Show all drop zones
            dropZones.forEach(zone => zone.classList.add('visible'));
        });

        item.addEventListener('dragend', (e) => {
            // console.log('Drag ended');
            item.classList.remove('dragging');

            // Hide all drop zones and remove drag-over states
            dropZones.forEach(zone => {
                zone.classList.remove('visible', 'drag-over');
            });
        });
    });

    // Setup drop zone events
    dropZones.forEach((zone) => {
        zone.addEventListener('dragover', (e) => {
            const hasContextIndex = e.dataTransfer.types.includes('application/x-context-index');

            if (hasContextIndex) {
                e.preventDefault();
                e.dataTransfer.dropEffect = 'move';
            }
        });

        zone.addEventListener('dragenter', (e) => {
            const hasContextIndex = e.dataTransfer.types.includes('application/x-context-index');

            if (hasContextIndex) {
                e.preventDefault();
                zone.classList.add('drag-over');
            }
        });

        zone.addEventListener('dragleave', (e) => {
            if (!zone.contains(e.relatedTarget)) {
                zone.classList.remove('drag-over');
            }
        });

        zone.addEventListener('drop', (e) => {
            const draggedIndex = parseInt(e.dataTransfer.getData('application/x-context-index'));
            const targetPosition = parseInt(zone.dataset.position);

            if (!isNaN(draggedIndex) && !isNaN(targetPosition)) {
                e.preventDefault();
                e.stopPropagation();
                // console.log('Dropping at position:', targetPosition, 'from index:', draggedIndex);

                moveContextToPosition(draggedIndex, targetPosition);
            }

            zone.classList.remove('drag-over');
        });
    });
}

function moveContextToPosition(fromIndex, targetPosition) {
    // console.log('Moving context from index', fromIndex, 'to position', targetPosition);
    // console.log('Before move:', currentContexts.map(c => c.path));

    // Get the sorted contexts to understand the current display order
    const sortedContexts = [...currentContexts].sort((a, b) => a.order - b.order);

    // Find which context we're moving
    const movingContext = currentContexts[fromIndex];
    const currentDisplayIndex = sortedContexts.indexOf(movingContext);

    // console.log('Moving context:', movingContext.path, 'from display position', currentDisplayIndex, 'to position', targetPosition);

    // Remove the moving item from its current position
    sortedContexts.splice(currentDisplayIndex, 1);

    // Insert at the target position
    const insertIndex = targetPosition > currentDisplayIndex ? targetPosition - 1 : targetPosition;
    sortedContexts.splice(insertIndex, 0, movingContext);

    // Update the order values based on new positions
    sortedContexts.forEach((context, index) => {
        context.order = index;
    });

    // console.log('After move:', sortedContexts.map(c => c.path));
    // console.log('New order values:', sortedContexts.map(c => c.order));

    renderContextList();
}

function removeContext(index) {
    currentContexts.splice(index, 1);
    currentContexts.forEach((c, i) => c.order = i);
    renderContextList();
}

async function saveNarrative() {
    const name = document.getElementById('narrative-name').value;
    const description = document.getElementById('narrative-description').value;
    const currentId = document.getElementById('current-narrative-id').value;
    const isTemplate = document.getElementById('is-template').checked;

    if (!name || currentContexts.length === 0) {
        alert('Please provide a name and at least one context item');
        return;
    }

    const narrative = {
        id: currentId || "00000000-0000-0000-0000-000000000000", // Use existing ID or dummy
        name,
        description: description || null,
        is_template: isTemplate,
        contexts: currentContexts,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
    };

    try {
        let response;
        if (currentId) {
            // Update existing narrative
            response = await fetch(`/api/narratives/${currentId}`, {
                method: 'PUT',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify(narrative)
            });
        } else {
            // Create new narrative
            response = await fetch('/api/narratives', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify(narrative)
            });
        }

        if (response.ok) {
            clearNarrativeBuilder();
            loadNarratives();
        }
    } catch (error) {
        console.error('Failed to save narrative:', error);
    }
}

async function saveAsNewNarrative() {
    const name = document.getElementById('narrative-name').value;
    const description = document.getElementById('narrative-description').value;
    const isTemplate = document.getElementById('is-template').checked;

    if (!name || currentContexts.length === 0) {
        alert('Please provide a name and at least one context item');
        return;
    }

    const narrative = {
        id: "00000000-0000-0000-0000-000000000000", // Always create new
        name,
        description: description || null,
        is_template: isTemplate,
        contexts: currentContexts,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
    };

    try {
        const response = await fetch('/api/narratives', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(narrative)
        });

        if (response.ok) {
            clearNarrativeBuilder();
            loadNarratives();
        }
    } catch (error) {
        console.error('Failed to save narrative:', error);
    }
}

function clearNarrativeBuilder() {
    document.getElementById('narrative-name').value = '';
    document.getElementById('narrative-description').value = '';
    document.getElementById('current-narrative-id').value = '';
    document.getElementById('is-template').checked = false;
    currentContexts = [];
    renderContextList();
}

function renderNarratives() {
    const narrativeList = document.getElementById('narrative-list');
    
    // Filter narratives based on current tab
    const filteredNarratives = narratives.filter(narrative => {
        const isTemplate = narrative.is_template || false;
        return currentTab === 'template' ? isTemplate : !isTemplate;
    });

    if (filteredNarratives.length === 0) {
        const tabName = currentTab === 'template' ? 'テンプレート' : '通常のナラティブ';
        narrativeList.innerHTML = `<p class="placeholder">No saved ${tabName}</p>`;
    } else {
        narrativeList.innerHTML = filteredNarratives.map(narrative => `
            <div class="narrative-item ${narrative.is_template ? 'template' : ''}">
                <h3>${narrative.name}${narrative.is_template ? '<span class="template-badge">テンプレート</span>' : ''}</h3>
                <div class="id">ID: ${narrative.id}</div>
                ${narrative.description ? `<p>${narrative.description}</p>` : ''}
                <div class="actions">
                    <button class="resolve" onclick="resolveNarrative('${narrative.id}')">Resolve</button>
                    <button class="copy-command" onclick="copySketchCommand('${narrative.id}', event)">Copy</button>
                    <button class="edit" onclick="editNarrative('${narrative.id}')">Edit</button>
                    <button class="delete" onclick="deleteNarrative('${narrative.id}')">Delete</button>
                </div>
            </div>
        `).join('');
    }
}

async function resolveNarrative(id) {
    try {
        const response = await fetch(`/api/narratives/${id}/resolve`);
        const content = await response.text();

        const blob = new Blob([content], {type: 'text/plain'});
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `narrative-${id}.txt`;
        a.click();
        URL.revokeObjectURL(url);
    } catch (error) {
        console.error('Failed to resolve narrative:', error);
    }
}

async function copySketchCommand(id, event) {
    // Find the narrative to check if it has a description
    const narrative = narratives.find(n => n.id === id);
    let command = `/sketch ${id}`;

    // If description exists, add it to the command
    if (narrative && narrative.description && narrative.description.trim()) {
        command += ` ${narrative.description}`;
    }

    try {
        await navigator.clipboard.writeText(command);
        // Visual feedback
        const button = event.target;
        const originalText = button.textContent;
        button.style.backgroundColor = '#2ecc71';
        button.textContent = 'Copied!';

        setTimeout(() => {
            button.textContent = originalText;
            button.style.backgroundColor = '#9b59b6';
        }, 1000);
    } catch (error) {
        console.error('Failed to copy to clipboard:', error);
        alert('Failed to copy to clipboard');
    }
}

async function editNarrative(id) {
    const narrative = narratives.find(n => n.id === id);
    if (!narrative) {
        console.error('Narrative not found:', id);
        return;
    }

    // Load narrative data into the builder
    document.getElementById('narrative-name').value = narrative.name;
    document.getElementById('narrative-description').value = narrative.description || '';
    document.getElementById('current-narrative-id').value = narrative.id;
    document.getElementById('is-template').checked = narrative.is_template || false;

    // Load contexts
    currentContexts = [...narrative.contexts];
    renderContextList();

    // Scroll to the narrative builder
    document.querySelector('.narrative-builder').scrollIntoView({behavior: 'smooth'});
}

async function deleteNarrative(id) {
    if (!confirm('Are you sure you want to delete this narrative?')) {
        return;
    }

    try {
        const response = await fetch(`/api/narratives/${id}`, {
            method: 'DELETE'
        });

        if (response.ok) {
            loadNarratives();
        }
    } catch (error) {
        console.error('Failed to delete narrative:', error);
    }
}

function switchTab(tab) {
    currentTab = tab;
    
    // Update tab button states
    document.querySelectorAll('.tab-button').forEach(button => {
        button.classList.remove('active');
        if (button.dataset.tab === tab) {
            button.classList.add('active');
        }
    });
    
    // Re-render narratives with new filter
    renderNarratives();
}

document.addEventListener('DOMContentLoaded', () => {
    loadFiles();
    loadNarratives();
    setupContextList();

    document.getElementById('save-narrative').addEventListener('click', saveNarrative);
    document.getElementById('save-as-new-narrative').addEventListener('click', saveAsNewNarrative);
    
    // Setup tab switching
    document.querySelectorAll('.tab-button').forEach(button => {
        button.addEventListener('click', () => {
            switchTab(button.dataset.tab);
        });
    });
});