package io.github.lxien.orbien.boot;

import io.github.lxien.orbien.client.OrbienClient;
import org.springframework.boot.autoconfigure.AutoConfiguration;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.context.annotation.Bean;

@AutoConfiguration
@EnableConfigurationProperties(OrbienProperties.class)
@ConditionalOnProperty(prefix = "orbien", name = "enabled", havingValue = "true", matchIfMissing = true)
public class OrbienAutoConfiguration {

    @Bean(destroyMethod = "close")
    @ConditionalOnMissingBean
    public OrbienClient orbienClient(OrbienProperties properties) {
        return new OrbienClient(properties.toClientConfig());
    }

    @Bean
    @ConditionalOnMissingBean
    public OrbienClientLifecycle orbienClientLifecycle(
            OrbienClient client, OrbienProperties properties) {
        return new OrbienClientLifecycle(client, properties);
    }
}
