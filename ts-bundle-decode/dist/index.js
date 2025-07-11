import init, { decode_bundle } from "../rs-bundle-decode/pkg/rs_bundle_decode.js";
export async function decodeBundle(encodedBundle) {
    await init();
    const bundleDecoded = decode_bundle(encodedBundle);
    const bundleObj = JSON.parse(bundleDecoded);
    return bundleObj;
}
