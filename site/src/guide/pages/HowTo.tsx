import { DocShell } from '../DocShell';
import { Section } from '../components';

export default function HowTo() {
  return (
    <DocShell page="howto" eyebrow="How-to guides" title="How-to guides" groups={['Stub']}>
      <Section id="stub" title="Stub" group="Stub">
        <p>Placeholder.</p>
      </Section>
    </DocShell>
  );
}
