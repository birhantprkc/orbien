import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';

import styles from './styles.module.css';

type FeatureItem = {
    title: string;
    description: string;
};

const FeatureList: FeatureItem[] = [
    {
        title: '高性能',
        description: '全链路零拷贝转发，低延迟、高吞吐、无GC停顿，适合长期服务器运行',
    },
    {
        title: '多协议代理',
        description:
            '支持 TCP、UDP、HTTP、HTTPS协议代理',
    },
    {
        title: '多传输协议',
        description:
            '支持 TCP、QUIC、KCP、WebSocket 多种传输协议，同时支持TCP多路复用'
    },
    {
        title: '安全加密',
        description:
            '支持 Token 鉴权与 TLS /mTLS 加密传输；HTTPS采用透明代理和可选客户端TLS终止',
    },
    {
        title: '可视化运维',
        description: '内置轻量服务端Web管理界面和桌面客户端，便于运维监控',
    },
    {
        title: '跨平台支持',
        description:
            '支持 Windows / macOS / Linux / freeBSD 等平台',
    },
];

function Feature({title, description}: FeatureItem): ReactNode {
    return (
        <div className={clsx('col col--4', styles.featureCol)}>
            <div className={styles.featureCard}>
                <Heading as="h3" className={styles.featureTitle}>
                    {title}
                </Heading>
                <p className={styles.featureDesc}>{description}</p>
            </div>
        </div>
    );
}

export default function HomepageFeatures(): ReactNode {
    return (
        <section className={styles.features}>
            <div className="container">
                <div className="row">
                    {FeatureList.map((item) => (
                        <Feature key={item.title} {...item} />
                    ))}
                </div>
            </div>
        </section>
    );
}
