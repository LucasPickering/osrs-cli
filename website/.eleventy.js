module.exports = function (eleventyConfig) {
  eleventyConfig.setBrowserSyncConfig({
    listen: "localhost", // https://github.com/BrowserSync/browser-sync/issues/352
    port: 3000,
  });

  eleventyConfig.addWatchTarget("./src/styles/");

  return {
    dir: {
      input: "src",
      output: "dist",
    },
  };
};
