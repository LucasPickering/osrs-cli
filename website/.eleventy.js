const markdownIt = require("markdown-it");
const markdownItAnchor = require("markdown-it-anchor");

module.exports = function (eleventyConfig) {
  eleventyConfig.setBrowserSyncConfig({
    listen: "localhost", // https://github.com/BrowserSync/browser-sync/issues/352
    port: 3000,
  });

  eleventyConfig.addPassthroughCopy("src/*.ico");
  eleventyConfig.addWatchTarget("./src/styles/");

  // Markdown Overrides
  const markdownLibrary = markdownIt({
    html: true,
    breaks: true,
    linkify: true,
  }).use(markdownItAnchor, {
    permalink: true,
    // permalinkClass: "direct-link",
    permalinkSymbol: "#",
  });
  eleventyConfig.setLibrary("md", markdownLibrary);

  return {
    dir: {
      input: "src",
      output: "dist",
    },
  };
};
