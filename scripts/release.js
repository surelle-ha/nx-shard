#!/usr/bin/env node

/**
 * Release Commit Script (ES Module)
 * Commits, tags, and pushes the release
 * Works cross-platform (Windows/Mac/Linux)
 */

import fs from "fs";
import path from "path";
import { execSync } from "child_process";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Read version from package.json
const packageJsonPath = path.join(__dirname, "..", "package.json");
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
const version = packageJson.version;
const tag = `v${version}`;

console.log(`Creating release for version: ${version}`);
console.log(`Tag: ${tag}`);

try {
  // Stage all changes
  console.log("\n📦 Staging changes...");
  execSync("git add .", { stdio: "inherit" });

  // Commit
  console.log(`\n💾 Committing changes...`);
  execSync(`git commit -m "Release ${tag}"`, { stdio: "inherit" });

  // Create tag
  console.log(`\n🏷️  Creating tag ${tag}...`);
  execSync(`git tag ${tag}`, { stdio: "inherit" });

  // Push with tags
  console.log(`\n🚀 Pushing to origin...`);
  execSync("git push origin main --tags", { stdio: "inherit" });

  console.log(`\n✅ Release ${tag} created successfully!`);
  console.log("\n📋 Next steps:");
  console.log("1. Go to GitHub Actions to monitor the build");
  console.log("2. Once complete, check your Releases page");
  console.log(`3. Download and test the installer`);
} catch (error) {
  console.error("\n❌ Error during release:", error.message);
  console.error("\nYou may need to manually run:");
  console.error("  git add .");
  console.error(`  git commit -m "Release ${tag}"`);
  console.error(`  git tag ${tag}`);
  console.error("  git push origin main --tags");
  process.exit(1);
}
