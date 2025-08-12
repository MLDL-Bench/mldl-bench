"""initial schema

Revision ID: 0001_initial
Revises: 
Create Date: 2025-08-12

"""
from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision = '0001_initial'
down_revision = None
branch_labels = None
depends_on = None


def upgrade() -> None:
    op.create_table(
        'devices',
        sa.Column('id', sa.String(length=36), primary_key=True),
        sa.Column('hostname', sa.String(length=255), nullable=False),
        sa.Column('os', sa.String(length=255), nullable=False),
        sa.Column('cpu_model', sa.String(length=255), nullable=True),
        sa.Column('cpu_cores', sa.Integer(), nullable=True),
        sa.Column('memory_gb', sa.Float(), nullable=True),
        sa.Column('gpu_model', sa.String(length=255), nullable=True),
        sa.Column('gpu_driver', sa.String(length=255), nullable=True),
        sa.Column('created_at', sa.DateTime(timezone=True), nullable=False, server_default=sa.func.now()),
    )

    op.create_table(
        'benchmarks',
        sa.Column('id', sa.String(length=36), primary_key=True),
        sa.Column('name', sa.String(length=255), nullable=False),
        sa.Column('version', sa.String(length=64), nullable=False),
        sa.UniqueConstraint('name', 'version', name='uq_benchmarks_name_version'),
    )

    op.create_table(
        'runs',
        sa.Column('id', sa.String(length=36), primary_key=True),
        sa.Column('device_id', sa.String(length=36), sa.ForeignKey('devices.id'), nullable=True),
        sa.Column('benchmark_id', sa.String(length=36), sa.ForeignKey('benchmarks.id'), nullable=True),
        sa.Column('started_at', sa.DateTime(timezone=True), nullable=False),
        sa.Column('completed_at', sa.DateTime(timezone=True), nullable=False),
        sa.Column('parameters', sa.JSON(), nullable=False),
        sa.Column('metrics', sa.JSON(), nullable=False),
    )

    op.create_table(
        'artifacts',
        sa.Column('id', sa.Integer(), primary_key=True, autoincrement=True),
        sa.Column('run_id', sa.String(length=36), sa.ForeignKey('runs.id'), nullable=False),
        sa.Column('path', sa.String(length=1024), nullable=False),
    )


def downgrade() -> None:
    op.drop_table('artifacts')
    op.drop_table('runs')
    op.drop_table('benchmarks')
    op.drop_table('devices')


