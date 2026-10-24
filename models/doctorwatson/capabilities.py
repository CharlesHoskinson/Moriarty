"""Pure fixed adapter facts shared by inspection and execution.

Availability describes implemented code, never source qualification, resource
admission, proof validity, or permission to launch an operation.
"""


def adapter_capability(adapter):
    """Return fresh capability facts; unknown adapters fail closed."""
    if adapter in ('moriarty-sdk-build', 'handkey-verify-pis'):
        return {'executor_available': True, 'collector_available': True,
                'blocked_reason': None}
    if adapter == 'sherlock-anchor-settle-readback':
        return {'executor_available': False, 'collector_available': True,
                'blocked_reason': 'adapter collector exists; executor unavailable'}
    return {'executor_available': False, 'collector_available': False,
            'blocked_reason': 'adapter executor and collector unavailable'}
