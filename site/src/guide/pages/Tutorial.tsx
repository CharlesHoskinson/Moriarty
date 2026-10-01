import { DocShell } from '../DocShell';
import { Section } from '../components';

export default function Tutorial() {
  return (
    <DocShell page="tutorial" eyebrow="Tutorials" title="Tutorials" groups={['Stub']}>
      <Section id="stub" title="Stub" group="Stub">
        <p>Placeholder.</p>
      </Section>
    </DocShell>
  );
}
