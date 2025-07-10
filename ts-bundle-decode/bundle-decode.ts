// Import the init function and decode_bundle
import init, { decode_bundle } from "../rs-bundle-decode/pkg/rs_bundle_decode.js";

const encodedBundle = "0x09c5eabe00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000174000088a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000001dd7e000000000000000000000000013dcd7600000000000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000000000000000000000000001bed63d818030b00002600000001000000000000000000000000000000000000000000000000000000000000000000000000330200000000000000000000001bed63d818030b0000000000000000000000000000000a0000000000000000000013806b76267d000084080000000000000000001bed63d818030b000000000000000000000000013dcd6c0000000000000000000000000003bafc0000000000000000000000000001dd7e00001ce7254b94c5bea4da8303c64c16dd07106247ca887e452f283f4a69d53c304a453f213d526e766320c829d2189748ab1a4ddd8c3d0ce12e96723b64ddae6fe6e8000000000000000000000000000000";

// Initialize WASM and decode
async function runDecode() {
    try {
        console.log("Initializing WASM module...");
        // Initialize the WASM module
        await init();
        console.log("WASM module initialized successfully");
        
        // Now we can call decode_bundle
        console.log("Decoding bundle...");
        const bundleDecoded = decode_bundle(encodedBundle);
        console.log("Bundle decoded:", bundleDecoded);
        
        // Parse and pretty print the JSON
        const bundleObj = JSON.parse(bundleDecoded);
        const prettyJson = JSON.stringify(bundleObj, null, 2);
        
        // Set the result onto the body
        document.body.innerHTML = `<pre>Decoded Bundle JSON:\n${prettyJson}</pre>`;
    } catch (error) {
        console.error("Error:", error);
        document.body.textContent = `Error: ${error}`;
    }
}

runDecode();