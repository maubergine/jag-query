// semantic-release config (.github/workflows/release.yml). Versions come from Conventional Commits: fix → patch,
// feat → minor, a breaking change → major. chore, ci, docs, test and refactor don't release.
//
// First release: tag the version already shipped (git tag v<x.y.z> on main) so numbering continues from there.
module.exports = {
  branches: ["main"],
  repositoryUrl: "https://github.com/maubergine/jag-query.git",
  tagFormat: "v${version}",
  plugins: [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    ["@semantic-release/changelog", { changelogFile: "CHANGELOG.md" }],
    // Keeps package.json's version in step. npmPublish stays false unless this is a published npm package.
    ["@semantic-release/npm", { npmPublish: false }],
    ["@semantic-release/exec", { prepareCmd: "sed -i.bak -E 's|^version = \"[^\"]*\"|version = \"${nextRelease.version}\"|' Cargo.toml && sed -i.bak '/^name = \"jag-query\"$/{n;s|^version = .*|version = \"${nextRelease.version}\"|;}' Cargo.lock && rm Cargo.toml.bak Cargo.lock.bak" }],
    [
      "@semantic-release/git",
      {
        assets: ["package.json", "package-lock.json", "CHANGELOG.md", "Cargo.toml", "Cargo.lock"],
        message: "chore(release): ${nextRelease.version} [skip ci]",
      },
    ],
    ["@semantic-release/github", {}],
  ],
};
