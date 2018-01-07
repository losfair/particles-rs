declare var process: {
    env: {
        NODE_ENV: string,
        PRS_BUILD_ID: string
    }
}

export default class Common {
    static code: Uint8Array = null;
    static buildId: string = process.env.PRS_BUILD_ID;
}
