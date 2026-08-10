import {useEffect, useMemo, useState, type ReactNode} from 'react';
import clsx from 'clsx';

import styles from './styles.module.css';

const REPO = 'orbien-org/orbien';

const FALLBACK_VERSION = '2.0.0';

type OsId = 'windows' | 'linux' | 'darwin' | 'freebsd';
type ArchId = 'amd64' | 'arm64';
type ProductId = 'orbien-server' | 'orbien' | 'orbien-desktop';
type AssetExt = 'tar.gz' | 'deb' | 'msi' | 'dmg';

type Row = {
    os: OsId;
    arch: ArchId;
    archLabel: string;
    note: string;
};

const RELEASE_BUILDS: ReadonlySet<string> = new Set([
    'orbien-server|linux|amd64',
    'orbien-server|linux|arm64',
    'orbien-server|windows|amd64',
    'orbien-server|windows|arm64',
    'orbien-server|darwin|amd64',
    'orbien-server|darwin|arm64',
    'orbien-server|freebsd|amd64',

    'orbien|linux|amd64',
    'orbien|linux|arm64',
    'orbien|windows|amd64',
    'orbien|windows|arm64',
    'orbien|darwin|amd64',
    'orbien|darwin|arm64',
    'orbien|freebsd|amd64',

    'orbien-desktop|linux|amd64',
    'orbien-desktop|linux|arm64',
    'orbien-desktop|windows|amd64',
    'orbien-desktop|windows|arm64',
    'orbien-desktop|darwin|amd64',
    'orbien-desktop|darwin|arm64',
]);

const ROWS: Row[] = [
    {os: 'linux', arch: 'amd64', archLabel: 'amd64', note: ''},
    {os: 'linux', arch: 'arm64', archLabel: 'arm64', note: ''},
    {os: 'freebsd', arch: 'amd64', archLabel: 'amd64', note: ''},
    {os: 'windows', arch: 'amd64', archLabel: 'x86_64', note: ''},
    {os: 'windows', arch: 'arm64', archLabel: 'arm64', note: ''},
    {
        os: 'darwin',
        arch: 'amd64',
        archLabel: 'x86_64',
        note: '如遇拦截，于系统设置中允许运行',
    },
    {
        os: 'darwin',
        arch: 'arm64',
        archLabel: 'arm64',
        note: '如遇拦截，于系统设置中允许运行',
    },
];

const OS_LABEL: Record<OsId, string> = {
    windows: 'Windows',
    darwin: 'macOS',
    linux: 'Linux',
    freebsd: 'FreeBSD',
};

const PRODUCTS = [
    {
        id: 'server',
        name: 'orbien-server' as ProductId,
        label: '服务端',
    },
    {
        id: 'client',
        name: 'orbien' as ProductId,
        label: '命令行客户端',
    },
    {
        id: 'desktop',
        name: 'orbien-desktop' as ProductId,
        label: '桌面客户端',
    },
] as const;

function buildKey(product: ProductId, os: OsId, arch: ArchId): string {
    return `${product}|${os}|${arch}`;
}

function isReleaseBuild(product: ProductId, os: OsId, arch: ArchId): boolean {
    return RELEASE_BUILDS.has(buildKey(product, os, arch));
}

function assetExt(product: ProductId, os: OsId): AssetExt {
    if (product !== 'orbien-desktop') {
        return 'tar.gz';
    }
    switch (os) {
        case 'linux':
            return 'deb';
        case 'windows':
            return 'msi';
        case 'darwin':
            return 'dmg';
        default:
            return 'tar.gz';
    }
}

function assetName(
    product: ProductId,
    version: string,
    os: OsId,
    arch: ArchId,
): string {
    return `${product}_${version}_${os}_${arch}.${assetExt(product, os)}`;
}

function assetUrl(filename: string, version: string): string {
    return `https://github.com/${REPO}/releases/download/v${version}/${filename}`;
}

function detectOs(): OsId | null {
    if (typeof navigator === 'undefined') {
        return null;
    }
    const ua = navigator.userAgent.toLowerCase();
    if (ua.includes('win')) {
        return 'windows';
    }
    if (ua.includes('mac')) {
        return 'darwin';
    }
    if (ua.includes('freebsd')) {
        return 'freebsd';
    }
    if (ua.includes('linux')) {
        return 'linux';
    }
    return null;
}

export default function DownloadMatrix(): ReactNode {
    const [version, setVersion] = useState(FALLBACK_VERSION);
    const [assetSet, setAssetSet] = useState<Set<string> | null>(null);
    const [detected, setDetected] = useState<OsId | null>(null);

    useEffect(() => {
        setDetected(detectOs());
    }, []);

    useEffect(() => {
        let cancelled = false;
        (async () => {
            try {
                const res = await fetch(
                    `https://api.github.com/repos/${REPO}/releases/latest`,
                    {headers: {Accept: 'application/vnd.github+json'}},
                );
                if (!res.ok) {
                    return;
                }
                const data = (await res.json()) as {
                    tag_name?: string;
                    assets?: { name: string }[];
                };
                if (cancelled) {
                    return;
                }
                const tag = data.tag_name?.replace(/^v/, '');
                if (tag) {
                    setVersion(tag);
                }
                if (data.assets) {
                    setAssetSet(new Set(data.assets.map((a) => a.name)));
                }
            } catch {

            }
        })();
        return () => {
            cancelled = true;
        };
    }, []);

    const osRowSpans = useMemo(() => {
        const spans = new Map<number, number>();
        let i = 0;
        while (i < ROWS.length) {
            const os = ROWS[i].os;
            let count = 1;
            while (i + count < ROWS.length && ROWS[i + count].os === os) {
                count += 1;
            }
            spans.set(i, count);
            i += count;
        }
        return spans;
    }, []);

    return (
        <div className={styles.wrap}>
            <div className={styles.toolbar}>
                <span className={styles.version}>v{version}</span>
                <a
                    className={styles.releasesLink}
                    href={`https://github.com/${REPO}/releases`}
                    rel="noopener noreferrer">
                    所有版本
                </a>
            </div>

            <div className={styles.tableScroll}>
                <table className={styles.table}>
                    <thead>
                    <tr>
                        <th>操作系统</th>
                        <th>架构</th>
                        {PRODUCTS.map((p) => (
                            <th key={p.id}>{p.label}</th>
                        ))}
                        <th>说明</th>
                    </tr>
                    </thead>
                    <tbody>
                    {ROWS.map((row, idx) => {
                        const span = osRowSpans.get(idx);
                        const showOs = span !== undefined;
                        return (
                            <tr
                                key={`${row.os}-${row.arch}`}
                                className={clsx(detected === row.os && styles.rowHighlight)}>
                                {showOs ? (
                                    <td rowSpan={span} className={styles.osCell}>
                                        {OS_LABEL[row.os]}
                                    </td>
                                ) : null}
                                <td>
                                    <code>{row.archLabel}</code>
                                </td>
                                {PRODUCTS.map((p) => {
                                    const built = isReleaseBuild(p.name, row.os, row.arch);
                                    const ext = assetExt(p.name, row.os);
                                    const file = assetName(p.name, version, row.os, row.arch);
                                    const published = assetSet ? assetSet.has(file) : true;
                                    const showLink = built && published;
                                    return (
                                        <td key={p.id}>
                                            {showLink ? (
                                                <a
                                                    className={styles.badge}
                                                    href={assetUrl(file, version)}
                                                    rel="noopener noreferrer">
                                                    {ext}
                                                </a>
                                            ) : (
                                                <span className={styles.missing} title={file}>
                                                    
                                                </span>
                                            )}
                                        </td>
                                    );
                                })}
                                <td className={styles.note}>{row.note}</td>
                            </tr>
                        );
                    })}
                    </tbody>
                </table>
            </div>

            <div className={styles.docker}>
                <h3>Docker</h3>
                <ul className={styles.dockerList}>
                    <li className={styles.dockerItem}>
                        <div className={styles.dockerMeta}>
                            <strong>orbien-server</strong>
                            <span className={styles.dockerTag}>server</span>
                        </div>
                        <code className={styles.dockerCmd}>
                            docker pull ghcr.io/orbien-org/orbien-server:latest
                        </code>
                    </li>
                    <li className={styles.dockerItem}>
                        <div className={styles.dockerMeta}>
                            <strong>orbien</strong>
                            <span className={styles.dockerTag}>client</span>
                        </div>
                        <code className={styles.dockerCmd}>
                            docker pull ghcr.io/orbien-org/orbien:latest
                        </code>
                    </li>
                </ul>
            </div>
        </div>
    );
}
