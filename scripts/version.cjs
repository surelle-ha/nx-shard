#!/usr/bin/env node

/**
 * Version Bump Script
 * Keeps version in sync across package.json, tauri.conf.json, and Cargo.toml
 *
 * Usage:
 *   node scripts/bump-version.js patch   # 1.0.0 -> 1.0.1
 *   node scripts/bump-version.js minor   # 1.0.0 -> 1.1.0
 *   node scripts/bump-version.js major   # 1.0.0 -> 2.0.0
 *   node scripts/bump-version.js 1.2.3   # Set to specific version
 */

const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

const versionType = process.argv[2];

if (!versionType) {
  console.error("Usage: node bump-version.js <patch|minor|major|x.y.z>");
  process.exit(1);
}

// Files to update
const packageJsonPath = path.join(__dirname, "..", "package.json");
const tauriConfPath = path.join(
  __dirname,
  "..",
  "src-tauri",
  "tauri.conf.json"
);
const cargoTomlPath = path.join(__dirname, "..", "src-tauri", "Cargo.toml");

// Read current version from package.json
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
const currentVersion = packageJson.version;

console.log(`Current version: ${currentVersion}`);

// Calculate new version
let newVersion;
if (versionType.match(/^\d+\.\d+\.\d+$/)) {
  // Specific version provided
  newVersion = versionType;
} else {
  // Use npm to calculate the new version
  try {
    newVersion = execSync(`npm version ${versionType} --no-git-tag-version`, {
      encoding: "utf8",
      cwd: path.dirname(packageJsonPath),
    })
      .trim()
      .replace(/^v/, "");
  } catch (error) {
    console.error("Error calculating version:", error.message);
    process.exit(1);
  }
}

console.log(`New version: ${newVersion}`);

// Update tauri.conf.json
console.log("Updating tauri.conf.json...");
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
tauriConf.version = newVersion;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + "\n");

// Update Cargo.toml
console.log("Updating Cargo.toml...");
let cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
cargoToml = cargoToml.replace(
  /^version\s*=\s*"[^"]*"/m,
  `version = "${newVersion}"`
);
fs.writeFileSync(cargoTomlPath, cargoToml);

// Update Cargo.lock
console.log("Updating Cargo.lock...");
try {
  execSync("cargo update --workspace", {
    cwd: path.dirname(cargoTomlPath),
    stdio: "inherit",
  });
} catch (error) {
  console.warn("Warning: Could not update Cargo.lock:", error.message);
}

console.log("\n✅ Version updated successfully!");
console.log("\nNext steps:");
console.log("1. Review the changes: git diff");
console.log(
  '2. Commit: git add . && git commit -m "Bump version to v' + newVersion + '"'
);
console.log("3. Tag: git tag v" + newVersion);
console.log("4. Push: git push origin main --tags");
console.log("\nOr run: npm run release");
