// script.js
document.addEventListener('DOMContentLoaded', () => {
  // Add toggle buttons to collapsible nodes
  const containers = document.querySelectorAll('.pt-function, .pt-function-body, .pt-binary, .pt-parenthesized, .pt-let');
  
  containers.forEach(container => {
    const toggle = document.createElement('span');
    toggle.className = 'pt-toggle';
    toggle.textContent = '▼';
    container.insertBefore(toggle, container.firstChild);
    container.classList.add('pt-container');
    
    const children = Array.from(container.children).filter(
      el => !el.classList.contains('pt-toggle')
    );
    
    if (children.length > 0) {
      const childrenWrapper = document.createElement('div');
      childrenWrapper.className = 'pt-children';
      children.forEach(child => childrenWrapper.appendChild(child.cloneNode(true)));
      
      container.appendChild(childrenWrapper);
      children.forEach(child => child.remove());
    }
  });

  // Handle toggle clicks
  document.querySelectorAll('.pt-toggle').forEach(toggle => {
    toggle.addEventListener('click', () => {
      const container = toggle.parentElement;
      container.classList.toggle('pt-collapsed');
      toggle.textContent = container.classList.contains('pt-collapsed') ? '▶' : '▼';
    });
  });

  // Create tooltip element
  const tooltip = document.createElement('div');
  tooltip.className = 'pt-tooltip';
  tooltip.style.display = 'none';
  document.body.appendChild(tooltip);

  // Show tooltip on node hover
  document.querySelectorAll('.pt-parsed-node').forEach(node => {
    node.addEventListener('mouseenter', (e) => {
      const rect = node.getBoundingClientRect();
      tooltip.style.left = `${rect.right + 10}px`;
      tooltip.style.top = `${rect.top}px`;
      tooltip.style.display = 'block';
      
      const spanStart = node.dataset.spanStart;
      const spanEnd = node.dataset.spanEnd;
      const text = node.dataset.text || '';
      
      let content = `
        <div class="pt-tooltip-header">Node Information</div>
        <div class="pt-tooltip-content">
          <div><strong>Span:</strong> ${spanStart} - ${spanEnd}</div>
          <div class="pt-tooltip-text">${escapeHtml(text)}</div>
      `;
      
      // Add error info if exists
      const error = node.querySelector('.pt-node-error, .pt-fatal-error');
      if (error) {
        content += `<div><strong>Error:</strong> ${error.dataset.message || ''}</div>`;
      }
      
      // Add node type
      const type = Array.from(node.classList).find(cls => 
        cls.startsWith('pt-') && 
        !['pt-parsed-node', 'pt-container', 'pt-collapsed'].includes(cls)
      );
      if (type) {
        content += `<div><strong>Type:</strong> ${type.replace('pt-', '')}</div>`;
      }
      
      content += `</div>`;
      tooltip.innerHTML = content;
    });

    node.addEventListener('mouseleave', () => {
      tooltip.style.display = 'none';
    });
  });
});

// Helper function to escape HTML
function escapeHtml(unsafe) {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}
