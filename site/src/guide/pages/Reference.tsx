import { DocShell } from '../DocShell';
import { Section } from '../components';

export default function Reference() {
  return (
    <DocShell page="reference" eyebrow="Reference" title="Reference" groups={['Stub']}>
      <Section id="stub" title="Stub" group="Stub">
        <p>Placeholder.</p>
      </Section>
    </DocShell>
  );
}
