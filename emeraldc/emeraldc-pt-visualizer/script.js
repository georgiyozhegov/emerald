  document.addEventListener('DOMContentLoaded', () => {
    // Mark nodes that have children as collapsible
    document.querySelectorAll('.pt-parsed-node').forEach(node => {
      if ([...node.children].filter(c => c.classList.contains('pt-parsed-node') || c.tagName === 'DIV').length > 1) {
        node.classList.add('has-children');
      }
      // Expand top-level nodes by default (improves visibility)
      if (node.parentElement.id === 'tree-window' || node.parentElement.classList.contains('pt-node')) {
        node.classList.add('expanded');
      }
    });
    // Expand/collapse toggle
    document.querySelectorAll('.pt-parsed-node.has-children > :first-child').forEach(firstChild => {
      firstChild.style.cursor = 'pointer';
      firstChild.addEventListener('click', e => {
        e.stopPropagation();
        const parent = firstChild.parentElement;
        parent.classList.toggle('expanded');
      });
    });

    // Render text content from data-text into the relevant child div
    document.querySelectorAll('.pt-parsed-node').forEach(node => {
      const dataText = node.getAttribute('data-text');
      if (!dataText) return;

      // For token containers, put text inside .pt-token/.pt-identifier/.pt-integer/.pt-binary-operator element
      const tokenChild =
        node.querySelector('.pt-token') ||
        node.querySelector('.pt-identifier') ||
        node.querySelector('.pt-integer') ||
        node.querySelector('.pt-binary-operator');

      if (tokenChild && tokenChild.textContent.trim() === '') {
        tokenChild.textContent = dataText.trim();
      } else if (
        !node.querySelector('.pt-token') &&
        !node.querySelector('.pt-identifier') &&
        !node.querySelector('.pt-integer') &&
        !node.querySelector('.pt-binary-operator') &&
        node.children.length === 0
      ) {
        // For leaves without children or token wrappers, show data-text directly
        node.textContent = dataText.trim();
      }
    });
  });
