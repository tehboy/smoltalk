// Import the wasm-bindgen glue code
import init, { eval_in_browser } from './pkg/browser_lib.js';

const codeArea = document.getElementById('code-area');
const outputArea = document.getElementById('output');
const runBrowserBtn = document.getElementById('run-browser');
const runServerBtn = document.getElementById('run-server');

// Initialize the Wasm module
async function main() {
    await init();

    // "Run in Browser" button
    runBrowserBtn.addEventListener('click', () => {
        const code = codeArea.value;
        const result = eval_in_browser(code); // Call our Rust function!
        outputArea.textContent = result;
    });

    // "Run on Server" button
    runServerBtn.addEventListener('click', async () => {
        const code = codeArea.value;
        const response = await fetch('/api/run', {
            method: 'POST',
            body: code
        });
        const result = await response.text();
        outputArea.textContent = result;
    });
}

main();