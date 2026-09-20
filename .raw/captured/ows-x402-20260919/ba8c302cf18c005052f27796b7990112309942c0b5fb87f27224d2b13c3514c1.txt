// Copy code block contents
function copyCode(button) {
  const wrapper = button.closest('.code-block-wrapper');
  const code = wrapper.querySelector('pre').textContent;
  navigator.clipboard.writeText(code).then(() => {
    const original = button.textContent;
    button.textContent = 'Copied!';
    setTimeout(() => { button.textContent = original; }, 2000);
  });
}

// Smooth scroll for anchor links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
  anchor.addEventListener('click', function(e) {
    const target = document.querySelector(this.getAttribute('href'));
    if (target) {
      e.preventDefault();
      target.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  });
});
