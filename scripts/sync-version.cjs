/**
 * Version Sync Script
 * 
 * Reads the version from package.json and syncs it to:
 * - src-tauri/Cargo.toml
 * - src-tauri/tauri.conf.json
 * 
 * Usage: npm run version:sync
 * 
 * Just edit the version in package.json, then run this script!
 */

const fs = require('fs');
const path = require('path');

const rootDir = path.resolve(__dirname, '..');

// Read version from package.json
const packageJsonPath = path.join(rootDir, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const version = packageJson.version;

console.log(`📦 Syncing version: ${version}`);

// Update tauri.conf.json
const tauriConfPath = path.join(rootDir, 'src-tauri', 'tauri.conf.json');
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
tauriConf.version = version;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');
console.log('✅ Updated tauri.conf.json');

// Update Cargo.toml
const cargoTomlPath = path.join(rootDir, 'src-tauri', 'Cargo.toml');
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
cargoToml = cargoToml.replace(/^version = ".*"$/m, `version = "${version}"`);
fs.writeFileSync(cargoTomlPath, cargoToml);
console.log('✅ Updated Cargo.toml');

console.log(`\n🎉 All files synced to version ${version}`);
