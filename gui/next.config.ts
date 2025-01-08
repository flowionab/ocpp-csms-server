import type {NextConfig} from "next";

const nextConfig: NextConfig = {
    /* config options here */
    experimental: {
        typedRoutes: true,
    },
    output: "standalone",
};

export default nextConfig;
