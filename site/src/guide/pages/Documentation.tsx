import { DocShell } from '../DocShell';
import { Section } from '../components';

export default function Documentation() {
  return (
    <DocShell page="documentation" eyebrow="Documentation" title="Documentation" groups={['Stub']}>
      <Section id="stub" title="Stub" group="Stub">
        <p>Placeholder.</p>
      </Section>
    </DocShell>
  );
}
