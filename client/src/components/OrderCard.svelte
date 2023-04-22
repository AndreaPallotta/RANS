<script lang="ts">
    import Card, { Content, Media, PrimaryAction } from '@smui/card';
    import type { IOrder } from '../types/models';
    import { formatNumberLiteral } from '../utils/utils';

    export let order: IOrder;

    const formattedDate = () => {
        const date = new Date(order.date);

        return new Intl.DateTimeFormat('en-US', {
            dateStyle: 'medium',
            timeStyle: 'short',
        }).format(date);
    };
</script>

<div class="card-display">
    <div class="card-container">
        <Card>
            <PrimaryAction on:click={() => {}}>
                <Media class="card-media-16x9" aspectRatio="16x9" />
                <Content class="mdc-typography--body2">
                    <h2 class="mdc-typography--headline6" style="margin: 0;">
                        {order.item_name}
                    </h2>
                    <span style="color: red;">Order ID: {order.item_id}</span>
                    <h4
                        class="mdc-typography--headline4"
                        id="description-label"
                    >
                        {formattedDate()}
                    </h4>

                    <div class="mdc-typography--headline6">
                        <b>Price:</b> ${order.price.toFixed(2)}
                    </div>
                    <div class="mdc-typography--headline6">
                        <b>Quantity:</b>
                        {formatNumberLiteral(order.quantity)}
                    </div>
                </Content>
            </PrimaryAction>
        </Card>
    </div>
</div>

<style>
    * :global(.card-media-16x9) {
        background-image: url(https://placehold.co/320x180?text=16x9);
    }
</style>
