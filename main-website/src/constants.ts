import { Release, Feature } from './types';
import { Trash2, RefreshCw, ShieldCheck, Zap, HardDrive, Layers } from 'lucide-vue-next';

export const MOCK_RELEASES: Release[] = [
  {
    id: 'rel_001',
    version: '1.2.0',
    name: 'Stable Release',
    description: 'Core engine rewrite in Rust. Optimized registry scanning algorithms and introduced new "Ghost File" detection for better deep cleaning.',
    date: '2023-11-05',
    downloadUrl: '#',
    isPrerelease: false,
    size: '3.4 MB',
    downloads_count: 15420,
  },
  {
    id: 'rel_002',
    version: '1.1.0',
    name: 'FOSS Engine Update',
    description: 'Added support for 500+ new open source alternatives. Improved UI responsiveness for large application lists.',
    date: '2023-09-15',
    downloadUrl: '#',
    isPrerelease: false,
    size: '3.2 MB',
    downloads_count: 8900,
  },
  {
    id: 'rel_003',
    version: '1.0.5',
    name: 'Maintenance Patch',
    description: 'Fixed high-DPI scaling issues on 4K monitors. Resolved a crash when uninstalling multiple Windows Store apps simultaneously.',
    date: '2023-08-01',
    downloadUrl: '#',
    isPrerelease: false,
    size: '3.1 MB',
    downloads_count: 4300,
  },
  {
    id: 'rel_004',
    version: '1.0.0',
    name: 'Initial Launch',
    description: 'First public stable release. Basic uninstaller, bulk actions, and disk analyzer features.',
    date: '2023-06-10',
    downloadUrl: '#',
    isPrerelease: false,
    size: '2.8 MB',
    downloads_count: 2100,
  },
];

export const FEATURES: Feature[] = [
  {
    title: 'Deep Clean',
    description: 'Scours filesystem and registry for zero-byte leftovers. Leave no trace behind.',
    icon: Trash2,
  },
  {
    title: 'FOSS Swap',
    description: 'Detects paid software. Suggests free, open-source alternatives instantly in the dashboard.',
    icon: RefreshCw,
  },
  {
    title: 'Privacy Guard',
    description: 'Blocks telemetry modules during uninstallation processes. Keep your data yours.',
    icon: ShieldCheck,
  },
  {
    title: 'Debloat Suite',
    description: 'Visual interface to remove pre-installed OEM and OS bloatware with a single click.',
    icon: Zap,
  },
  {
    title: 'Disk Mapper',
    description: 'Interactive visualization of storage clusters to identify heavy applications.',
    icon: HardDrive,
  },
  {
    title: 'Native Core',
    description: 'Built with Rust and WinAPI. No Electron, no webview lag, instant startup.',
    icon: Layers,
  },
];
