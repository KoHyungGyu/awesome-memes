/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: true,
  // Note: This feature is required to use NextJS Image in SSG mode.
  // https://nextjs.org/docs/messages/export-image-api 에서 또다른 해결책을 확인할 수 있습니다.
  images: {
    unoptimized: true,
  },
}

module.exports = nextConfig
