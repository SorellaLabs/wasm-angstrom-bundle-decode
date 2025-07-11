import init, { decode_bundle } from "./pkg/rs_bundle_decode.js";

export async function decodeBundle(encodedBundle: string): Promise<any> {
    await init();
    const bundleDecoded = decode_bundle(encodedBundle);
    const bundleObj = JSON.parse(bundleDecoded);
    return bundleObj;
}

