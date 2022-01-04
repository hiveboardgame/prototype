import { Box, Flex } from '@chakra-ui/react';
import { HexCoordinate, StackCoordinate, TileId } from 'hive-lib';
import { throttle } from 'lodash';
import { PropsWithChildren, useEffect, useRef } from 'react';
import { useElementSize } from 'usehooks-ts';
import { BugSymbols } from './BugSymbols';
import { Hive } from './Hive';
import { ValidMovesOverlay } from './ValidMovesOverlay';
import {
  applyToPoint,
  identity,
  inverse,
  toString,
  transform,
  translate
} from 'transformation-matrix';

interface TableProps {
  hexSize: number;
  tilePadding: number;
  stacks?: StackCoordinate[];
  selectedTileId?: TileId;
  validMoves?: HexCoordinate[];
  onClickTable?: () => void;
  onClickTableStack?: (coordinate: HexCoordinate, stack: TileId[]) => void;
  onClickGhost?: (coordinate: HexCoordinate) => void;
  boardCentered?: string;
}

const Table = (props: PropsWithChildren<TableProps>) => {
  const {
    children,
    hexSize,
    tilePadding,
    stacks,
    selectedTileId,
    validMoves,
    onClickTable,
    onClickTableStack,
    onClickGhost,
    boardCentered
  } = props;

  const svgRef = useRef<SVGSVGElement>(null);
  const gRef = useRef<SVGGElement>(null);
  const [boxRef, { width, height }] = useElementSize();
  const viewBox = svgRef.current
    ? `-${width / 2 - 200} -${height / 2 - 32} ${width} ${height}`
    : undefined;

  useEffect(() => {
    const svg = svgRef.current;
    const g = gRef.current;
    if (svg && g) {
      // create svg points to calculate drag distance and a transformation
      // matrix to carry the transform
      let refPt = svg.createSVGPoint();
      let dragPt = svg.createSVGPoint();
      let matrix = identity();

      // create a flag we can use to indicate we're dragging
      let dragging = false;

      // when the mouse is downed, un-apply any existing matrix transforms to
      // get the actual click coordinates and keep track of the location as our
      // reference point
      const onMouseDown = (event: MouseEvent) => {
        event.preventDefault();

        const newRefPt = applyToPoint(inverse(matrix), {
          x: event.clientX,
          y: event.clientY
        });
        refPt.x = newRefPt.x;
        refPt.y = newRefPt.y;
        refPt = refPt.matrixTransform(svg.getScreenCTM()!.inverse());
        dragging = true;
      };

      // when the mouse is released, unset the dragging flag
      const onMouseUp = () => {
        dragging = false;
      };

      // when the mouse is dragged, get the point where the event occurred in
      // local coordinates and calculate the transformation matrix based on the
      // difference between that point and the reference point.
      const onDrag = (event: MouseEvent) => {
        if (dragging) {
          event.preventDefault();

          dragPt.x = event.clientX;
          dragPt.y = event.clientY;
          const ctm = svg.getScreenCTM();
          if (ctm) {
            dragPt = dragPt.matrixTransform(svg.getScreenCTM()!.inverse());
            matrix = transform(
              translate(dragPt.x - refPt.x, dragPt.y - refPt.y)
            );

            // convert the matrix to a string and set as the svg transform prop
            g.setAttribute('transform', toString(matrix));
          }
        }
      };

      const onDragThrottled = throttle(onDrag, 16, {
        leading: true,
        trailing: false
      });

      // prevent the context menu on right-click
      const onContextMenu = (event: MouseEvent) => {
        event.preventDefault();
      };

      // add the mouse event listeners, throttling the drag event
      svg.addEventListener('mousedown', onMouseDown, true);
      svg.addEventListener('mouseup', onMouseUp, true);
      svg.addEventListener('contextmenu', onContextMenu, true);
      svg.addEventListener('mousemove', onDragThrottled, true);

      g.setAttribute('transform', toString(matrix));

      return () => {
        svg.removeEventListener('mousedown', onMouseDown, true);
        svg.removeEventListener('mouseup', onMouseUp, true);
        svg.removeEventListener('contextmenu', onContextMenu, true);
        svg.removeEventListener('mousemove', onDragThrottled, true);
      };
    }
  }, [boardCentered]);

  return (
    <Flex width='full' height='full'>
      <Box ref={boxRef} width='full' height='full'>
        <svg
          ref={svgRef}
          width={width}
          height={height}
          viewBox={viewBox}
          onClick={onClickTable}
        >
          <g ref={gRef}>
            <BugSymbols />
            {stacks && (
              <Hive
                stacks={stacks}
                hexSize={hexSize}
                tilePadding={tilePadding}
                selectedTileId={selectedTileId}
                onClick={onClickTableStack}
              />
            )}
            {validMoves && (
              <ValidMovesOverlay
                coordinates={validMoves}
                hexSize={hexSize}
                onClick={onClickGhost}
              />
            )}
            {children}
          </g>
        </svg>
      </Box>
    </Flex>
  );
};

export { Table };
