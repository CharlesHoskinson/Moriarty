import { DocShell } from '../DocShell';
import { Section } from '../components';

export default function Explanation() {
  return (
    <DocShell page="explanation" eyebrow="Explanation" title="Explanation" groups={['Stub']}>
      <Section id="stub" title="Stub" group="Stub">
        <p>Placeholder.</p>
      </Section>
    </DocShell>
  );
}
