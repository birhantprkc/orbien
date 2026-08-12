import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useBaseUrl from '@docusaurus/useBaseUrl';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import ThemedImage from '@theme/ThemedImage';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

import styles from './index.module.css';

function HeroDashboard(): ReactNode {
    const lightSrc = useBaseUrl('/img/dashboard.png');
    const darkSrc = useBaseUrl('/img/dashboard_black.png');

    return (
        <div className={styles.heroShot}>
            <div className={styles.heroShotGlow} aria-hidden="true" />
            <ThemedImage
                className={styles.heroShotImg}
                alt="Orbien 项目监控面板"
                width={1492}
                height={835}
                sources={{
                    light: lightSrc,
                    dark: darkSrc,
                }}
            />
        </div>
    );
}

function HomepageHeader(): ReactNode {
    return (
        <header className={styles.hero}>
            <div className={styles.heroInner}>
                <div className={styles.heroCopy}>
                    <Heading as="h1" className={styles.heroBrand}>
                        <span className={styles.brandOrb}>Orb</span>
                        <span className={styles.brandRest}>ien</span>
                    </Heading>
                    <p className={styles.heroTagline}>由 Rust 与 Tokio 驱动</p>
                    <p className={styles.heroDesc}>
                        轻量、高性能、安全的内网穿透与反向代理，二进制体积5MB左右
                    </p>
                    <div className={styles.heroActions}>
                        <Link
                            className={clsx('button button--lg', styles.btnPrimary)}
                            to="/docs/intro">
                            快速开始
                        </Link>
                        <Link
                            className={clsx('button button--lg', styles.btnSecondary)}
                            to="/docs/download">
                            下载
                        </Link>
                        <Link
                            className={clsx('button button--lg', styles.btnGitHub)}
                            href="https://github.com/orbien-org/orbien">
                            GitHub
                        </Link>
                    </div>
                </div>
                <div className={styles.heroVisual}>
                    <HeroDashboard />
                </div>
            </div>
        </header>
    );
}

function DesktopShowcase(): ReactNode {
    const gifSrc = useBaseUrl('/img/desktop.gif');

    return (
        <section className={styles.desktop} aria-labelledby="desktop-showcase-title">
            <div className={styles.desktopInner}>
                <div className={styles.desktopCopy}>
                    <Heading as="h2" id="desktop-showcase-title" className={styles.desktopTitle}>
                        桌面客户端
                    </Heading>
                    <p className={styles.desktopDesc}>
                        通过可视化管理界面轻松管理穿透配置
                    </p>
                    <Link
                        className={clsx('button button--lg', styles.btnSecondary)}
                        to="/docs/download">
                        下载桌面端
                    </Link>
                </div>
                <div className={styles.desktopVisual}>
                    <div className={styles.desktopFrame}>
                        <img
                            className={styles.desktopGif}
                            src={gifSrc}
                            alt="Orbien Desktop 客户端演示"
                            width={1920}
                            height={1279}
                            loading="lazy"
                            decoding="async"
                        />
                    </div>
                </div>
            </div>
        </section>
    );
}

export default function Home(): ReactNode {
    const {siteConfig} = useDocusaurusContext();
    return (
        <Layout
            title={`${siteConfig.title} — 内网穿透与反向代理`}
            description="Orbien：简单、安全的内网穿透与反向代理，支持多协议代理与多传输协议">
            <HomepageHeader />
            <main>
                <HomepageFeatures />
                <DesktopShowcase />
            </main>
        </Layout>
    );
}
